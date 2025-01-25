mod visitors;

use std::collections::BTreeMap;

use tracing::span;
use tracing_subscriber::prelude::*;
use tracing_subscriber::Layer;
use visitors::{JsonVisitor, PrintlnVisitor};

#[derive(Debug)]
pub struct SFieldStorage(BTreeMap<String, serde_json::Value>);
pub struct ServerLogLayer;

impl<S> Layer<S> for ServerLogLayer
where
    S: tracing::Subscriber,
    S: for<'lookup> tracing_subscriber::registry::LookupSpan<'lookup>,
{
    fn on_new_span(
        &self,
        attrs: &span::Attributes<'_>,
        id: &span::Id,
        ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        let span_buffer = BTreeMap::new();
        let span = ctx.span(id).unwrap();
        println!("new_span");
        println!("  name={}", span.name());
        println!("  target={}", span.metadata().target());
        println!("  fields={:?}", span.fields());
        println!();

        let mut visitor = PrintlnVisitor;
        attrs.record(&mut visitor);

        let storage = SFieldStorage(span_buffer);
        let span = ctx.span(id).unwrap();
        let mut extensions = span.extensions_mut();
        extensions.insert::<SFieldStorage>(storage);
    }

    fn on_event(&self, event: &tracing::Event<'_>, ctx: tracing_subscriber::layer::Context<'_, S>) {
        let parent_span = ctx.event_span(event).unwrap();
        println!("span");
        println!("  name={}", parent_span.name());
        println!("  target={}", parent_span.metadata().target());

        let scope = ctx.event_scope(event).unwrap();
        for span in scope.from_root() {
            println!("scope");
            println!("  name={}", span.name());
            println!("  target={}", span.metadata().target());
        }
        println!();

        // let buffer = &mut BTreeMap::new();
        // let mut visitor = JsonVisitor(buffer);
        // event.record(&mut visitor);

        // let output = serde_json::json!({
        //     "target": event.metadata().target(),
        //     "event": event.metadata().name(),
        //     "level": event.metadata().level().to_string(),
        //     "fields": buffer
        // });

        // println!("{}", serde_json::to_string_pretty(&output).unwrap());
    }
}
