use std::future::Future;

mod database;

fn with_stack_size<F: Future>(test: F) {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .thread_stack_size(32 * 1024 * 1024)
        .build()
        .unwrap()
        .block_on(test);
}
