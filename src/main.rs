mod app;
mod icons;
mod theme_io;

use tracing_subscriber::{fmt, EnvFilter};

fn main() -> cosmic::iced::Result {
    fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();
    cosmic::applet::run::<app::App>(())
}
