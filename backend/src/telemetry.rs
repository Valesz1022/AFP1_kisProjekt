//! Modul az alapvető naplózási megoldásokhoz.
//! Tracing, `tracing_subscriber` crate-ket használva egyszerűvé teszi a
//! végpontokban való naplózást.

use tracing_subscriber::{
    fmt::{self, format::FmtSpan},
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter,
};

/// A naplózáshoz szükséges feliratkozó (Subscriber) létrehozása, és
/// konfigurálása. Amint ez létrehozásra került, hallgatni fogja az eseményeket,
/// és naplózni a konzolra (stdout) olvasható formátumban.
///
/// Alapvetően környezeti változóból keresi a naplózáshoz szükséges beállításokat,
/// de ha ez nem található meg, akkor a következő beállításokat alkalmazza:
/// jelenlegi crate: debug, `tower_http`: debug, `axum::rejection`: trace. Így
/// csak az ezek, vagy ezeken felüli naplózási események kerülnek megjelenítésre
/// az stdout-on.
#[inline]
pub fn init() {
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            format!(
                "{}=debug,tower_http=debug,axum::rejection=trace",
                env!("CARGO_CRATE_NAME"),
            )
            .into()
        }))
        .with(fmt::layer().with_span_events(FmtSpan::NEW | FmtSpan::CLOSE))
        .init();
}
