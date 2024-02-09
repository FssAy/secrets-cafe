use std::future::Future;

mod database;

/// Runs an async test with a new tokio runtime with higher stack size.
///
/// This is required to test SurrealDB.
/// If not used, test on SurrealDB might overflow its stack.
// This is not a great solution, but it's the best I've came to.
// If you have a better idea, pls fix <3
fn with_stack_size<F: Future>(test: F) {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .thread_stack_size(32 * 1024 * 1024)
        .build()
        .unwrap()
        .block_on(test);
}
