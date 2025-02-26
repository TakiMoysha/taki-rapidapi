mod visitors;

use std::collections::BTreeMap;

use tracing::span;
use tracing::Instrument;
use tracing_subscriber::prelude::*;
use tracing_subscriber::Layer;
use visitors::{JsonVisitor, PrintlnVisitor};

#[derive(Default, Debug)]
pub struct ServerLogLayer;

#[derive(Debug)]
pub struct ServerFieldStorage(BTreeMap<String, serde_json::Value>);

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
        let mut buffer = BTreeMap::new();
        let mut visitor = JsonVisitor(&mut buffer);
        attrs.record(&mut visitor);
        let storage = ServerFieldStorage(buffer);
        let span = ctx.span(id).unwrap();
        let mut extensions = span.extensions_mut();
        extensions.insert::<ServerFieldStorage>(storage);
    }

    fn on_event(&self, event: &tracing::Event<'_>, ctx: tracing_subscriber::layer::Context<'_, S>) {
        let scope = ctx.event_scope(event).expect("event has no scope()");
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
            "spans": spans,
        });

        println!("{}", serde_json::to_string_pretty(&output).unwrap());
    }

    fn on_record(
        &self,
        span: &span::Id,
        values: &span::Record<'_>,
        ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        let span = ctx.span(span).unwrap();

        let mut extensions_mut = span.extensions_mut();
        let storage = extensions_mut.get_mut::<ServerFieldStorage>().unwrap();
        let json_data: &mut BTreeMap<String, serde_json::Value> = &mut storage.0;
        let mut visitor = JsonVisitor(json_data);
        values.record(&mut visitor);
    }
}
