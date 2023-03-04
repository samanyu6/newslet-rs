use tracing::{subscriber::set_global_default, Subscriber};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{
    fmt::MakeWriter, prelude::__tracing_subscriber_SubscriberExt, EnvFilter, Registry,
};

pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    // allows logs to be traced by subscriber
    LogTracer::init().expect("Failure to set logger");

    // can be used by apps to specify what subscriber processes the span
    set_global_default(subscriber).expect("failed to set subscriber");
}

// return impl so send and sync are abstracted from us and we're simply passing it into init
pub fn get_subscriber<Sink>(
    name: String,
    env_filter: String,
    sink: Sink,
) -> impl Subscriber + Send + Sync
where
    // higher ranked trait bound: means that sink implements makewriter for all choices of lifetime ,<'a>
    Sink: for<'a> MakeWriter<'a> + 'static + Send + Sync,
{
    // enables logging level as an env var if env var isn't present
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let formatting_layer = BunyanFormattingLayer::new(
        name, // output into cli
        sink,
    );

    // register our tracing subscriber into a single subscriber through layers
    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
}
