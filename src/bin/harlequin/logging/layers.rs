mod visitors;

use std::collections::BTreeMap;

use tracing::span;
use tracing_subscriber::prelude::*;
use tracing_subscriber::Layer;
use visitors::{JsonVisitor, PrintlnVisitor};

#[derive(Debug)]
pub struct ServerFieldStorage(BTreeMap<String, serde_json::Value>);
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
        // println!("new_span");
        // println!("  name={}", span.name());
        // println!("  target={}", span.metadata().target());
        // println!("  fields={:?}", span.fields());
        // println!();

        let mut visitor = PrintlnVisitor;
        attrs.record(&mut visitor);

        let storage = ServerFieldStorage(span_buffer);
        let span = ctx.span(id).unwrap();
        let mut extensions = span.extensions_mut();
        extensions.insert::<ServerFieldStorage>(storage);
    }

    fn on_event(&self, event: &tracing::Event<'_>, ctx: tracing_subscriber::layer::Context<'_, S>) {
        let scope = ctx.event_scope(event).unwrap();
        let mut spans = vec![];

        for span in scope.from_root() {
            let extensions = span.extensions();
            let storage = extensions.get::<ServerFieldStorage>().unwrap();
            let field_data: &BTreeMap<String, serde_json::Value> = &storage.0;
            spans.push(serde_json::json!({
                "name": span.name(),
                "target": span.metadata().target(),
                "level": format_args!("{}", span.metadata().level()),
                "fields": field_data
            }));
        }

        let mut buf = BTreeMap::new();
        let mut visitor = JsonVisitor(&mut buf);
        event.record(&mut visitor);

        let output = serde_json::json!({
            "target": event.metadata().target(),
            "event": event.metadata().name(),
            "level": event.metadata().level().to_string(),
            "fields": buf,
        });

        println!("{}", serde_json::to_string_pretty(&output).unwrap());
    }
}
