use std::future::Future;

use once_cell::sync::Lazy;
use tokio_util::context::TokioContext;

pub fn spawn(f: impl Future<Output = ()> + Send + 'static) {
    EXECUTOR.spawn(f);
}

struct ThreadPool {
    inner: futures::executor::ThreadPool,
    rt: tokio::runtime::Runtime,
}

static EXECUTOR: Lazy<ThreadPool> = Lazy::new(|| {
    // Spawn tokio runtime on a single background thread
    // enabling IO and timers.
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(100)
        .enable_all()
        .build()
        .unwrap();
    let inner = futures::executor::ThreadPool::builder().create().unwrap();

    ThreadPool { inner, rt }
});

impl ThreadPool {
    fn spawn(&self, f: impl Future<Output = ()> + Send + 'static) {
        let handle = self.rt.handle().clone();
        self.inner.spawn_ok(TokioContext::new(f, handle));
    }
}
