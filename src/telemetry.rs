use tracing::Subscriber;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{fmt::MakeWriter, layer::SubscriberExt, EnvFilter, Registry};

pub fn get_subscriber<S>(
    name: String,
    default_env_filter: String,
    sink: S,
) -> impl Subscriber + Send + Sync
where
    // higher-ranked trait bound (HRTB), meaning S implements the `MakeWriter`
    // trait for all choices of the lifetime parameter `'a`
    S: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    // Default to info-level logging, if if the `RUST_LOG` environment variable has not been set.
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(default_env_filter));
    let formatting_layer = BunyanFormattingLayer::new(name, sink);
    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
}

pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    // Redirect all `log`'s events to our subscriber
    LogTracer::init().expect("Failed to set logger");
    // `set_global_default` can be used by applications to specify what subscriber should be
    // used to process spans.
    tracing::subscriber::set_global_default(subscriber).expect("Failed to set subscriber");
}
