use once_cell::sync::Lazy;
use reqwest::header::{ACCEPT, CONTENT_TYPE, HeaderMap, HeaderValue, USER_AGENT};
use tokio::sync::Mutex;

trait WithLock<T> {
    fn with_lock<R, F: FnOnce(&mut T) -> R>(&self, f: F) -> R;
}

impl<T> WithLock<T> for std::sync::Mutex<T> {
    fn with_lock<R, F: FnOnce(&mut T) -> R>(&self, f: F) -> R {
        let mut guard = self.lock().unwrap();
        f(&mut guard)
    }
}

#[async_trait::async_trait]
pub trait AsyncWithLock<T> {
    async fn async_with_lock<R, F: FnOnce(&mut T) -> R + Send>(&self, f: F) -> R;
}

#[async_trait::async_trait]
impl<T: Send> AsyncWithLock<T> for Mutex<T> {
    async fn async_with_lock<R, F: FnOnce(&mut T) -> R + Send>(&self, f: F) -> R {
        let mut guard = self.lock().await;
        f(&mut *guard)
    }
}

pub static SWGOHGG_CLIENT: Lazy<tokio::sync::Mutex<reqwest::Client>> = Lazy::new(|| {
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("rust_app/1.0.0"));
    headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    tokio::sync::Mutex::new(
        reqwest::Client::builder()
            .http2_prior_knowledge()
            .cookie_store(true)
            .default_headers(headers)
            .build()
            .unwrap(),
    )
});
