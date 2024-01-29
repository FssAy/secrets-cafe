use tracing_subscriber::FmtSubscriber;

pub fn init() {
    FmtSubscriber::builder()
        .with_ansi(enable_ansi())
        .with_env_filter(make_filter())
        .without_time()
        .init();

    #[cfg(debug_assertions)]
    warn!("Running in the debug mode!");

    0;
}

fn enable_ansi() -> bool {
    #[cfg(windows)]
    return ansi_term::enable_ansi_support().is_ok();

    #[cfg(not(windows))]
    return true;
}

fn make_filter() -> String {
    let get_level = || {
        #[cfg(debug_assertions)]
        return "debug";
        #[cfg(not(debug_assertions))]
        return "info";
    };

    format!(
        "{}={}",
        "secrets_cafe", get_level(),
    )
}
