use tracing_subscriber::FmtSubscriber;

/// Initializes the `tracing` crate.
pub fn init() {
    FmtSubscriber::builder()
        .with_ansi(enable_ansi())
        .with_env_filter(make_filter())
        .without_time()
        .init();

    #[cfg(debug_assertions)]
    warn!("Running in the debug mode! This might cause stack overflow when performing database operations.");
    // SurrealDB has severe issues when running in the debug mode and requires higher stack size.
    // It might be a good idea to increase it when running the debug mode.
}

/// Enables ANSI support on Windows.
/// For linux based systems it's unnecessary.
fn enable_ansi() -> bool {
    #[cfg(windows)]
    return ansi_term::enable_ansi_support().is_ok();

    #[cfg(not(windows))]
    return true;
}

/// Creates a logging filter to show only certain type of messages.
fn make_filter() -> String {
    let get_level = || {
        #[cfg(debug_assertions)]
        return "debug";
        #[cfg(not(debug_assertions))]
        return "info";
    };

    format!(
        "{}={},{}={}",
        "secrets_cafe", get_level(),
        "surrealdb", "warn",
    )
}
