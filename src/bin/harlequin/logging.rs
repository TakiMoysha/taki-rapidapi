use std::env;
// use tracing::subscriber::set_global_default;
// use tracing_subscriber::filter::EnvFilter;
use tracing::info;
use tracing::info_span;
use tracing_subscriber::prelude::*;
use tracing_subscriber::Layer;

mod layers;

pub fn setup_logging() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(layers::ServerLogLayer)
        .init();

    let outer_span = info_span!("outer", level = 0);
    let _outter_entered = outer_span.enter();

    let inner_span = info_span!("inner", level = 0);
    let _inner_entered = inner_span.enter();

    info!(pretty = true, answer = 42, message = "example");
    info!(pretty = false, answer = 42, message = "example");
    info!(answer = 42, message = "example");

    // let filter = match env::var("RUST_LOG") {
    //     Ok(_) => EnvFilter::from_env("RUST_LOG"),
    //     _ => EnvFilter::new("template=info"),
    // };
    // let fmt = tracing_subscriber::fmt::Layer::default();
    // let subscriber = filter.and_then(fmt).with_subscriber(Registry::default());
    // set_global_default(subscriber)?;
    Ok(())
}
