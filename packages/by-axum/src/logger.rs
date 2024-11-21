use slog::{Drain, OwnedKV, SendSyncRefUnwindSafeKV};
use static_str_ops::staticize;
use std::result;

pub struct RuntimeLevelFilter<D> {
    pub drain: D,
}

impl<D> Drain for RuntimeLevelFilter<D>
where
    D: Drain,
{
    type Ok = Option<D::Ok>;
    type Err = Option<D::Err>;

    fn log(
        &self,
        record: &slog::Record,
        values: &slog::OwnedKVList,
    ) -> result::Result<Self::Ok, Self::Err> {
        if record.level().is_at_least(match option_env!("LOG_LEVEL") {
            Some("trace") => slog::Level::Trace,
            Some("debug") => slog::Level::Debug,
            Some("info") => slog::Level::Info,
            Some("warn") => slog::Level::Warning,
            Some("error") => slog::Level::Error,
            Some("critical") => slog::Level::Critical,
            _ => slog::Level::Info,
        }) {
            self.drain.log(record, values).map(Some).map_err(Some)
        } else {
            Ok(None)
        }
    }
}

pub fn root() -> slog::Logger {
    let decorator = slog_term::PlainSyncDecorator::new(std::io::stdout());
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = RuntimeLevelFilter { drain }.fuse();

    slog::Logger::root(drain, slog::o!("service" => "dagit"))
}

pub fn new<T>(values: OwnedKV<T>) -> slog::Logger
where
    T: SendSyncRefUnwindSafeKV + 'static,
{
    let root = root();
    root.new(values)
}

pub fn new_api(method: &'static str, api: &str) -> slog::Logger {
    let api = staticize(api);

    new(slog::o!("API Method" => method, "API Endpoint" => api))
}
