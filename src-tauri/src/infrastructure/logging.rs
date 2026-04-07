use once_cell::sync::OnceCell;
use tracing_subscriber::{fmt, EnvFilter};

static LOGGING_INIT: OnceCell<()> = OnceCell::new();

pub fn init_logging() {
    LOGGING_INIT.get_or_init(|| {
        let filter = EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| EnvFilter::new("info,tao=warn,wry=warn"));

        fmt()
            .with_env_filter(filter)
            .with_target(false)
            .compact()
            .init();
    });
}
