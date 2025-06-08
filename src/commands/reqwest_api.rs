use once_cell::sync::Lazy;
use reqwest::header::{ACCEPT, CONTENT_TYPE, HeaderMap, HeaderValue, USER_AGENT};

trait WithLock<T> {
    fn with_lock<R, F: FnOnce(&mut T) -> R>(&self, f: F) -> R;
}

impl<T> WithLock<T> for std::sync::Mutex<T> {
    fn with_lock<R, F: FnOnce(&mut T) -> R>(&self, f: F) -> R {
        let mut guard = self.lock().unwrap();
        f(&mut guard)
    }
}

pub static SWGOHGG_CLIENT: Lazy<tokio::sync::Mutex<reqwest::Client>> = Lazy::new(|| {
    tokio::sync::Mutex::new(
        reqwest::Client::builder()
            .http2_prior_knowledge()
            .cookie_store(true)
            .build()
            .unwrap(),
    )
});

pub static SWGOHGG_HEADERS: Lazy<std::sync::Mutex<HeaderMap>> =
    Lazy::new(|| std::sync::Mutex::new(HeaderMap::new()));

pub fn init_headers() {
    SWGOHGG_HEADERS.with_lock(|gaurd| {
        gaurd.insert(USER_AGENT, HeaderValue::from_static("rust_app/1.0.0"));
        gaurd.insert(ACCEPT, HeaderValue::from_static("application/json"));
        gaurd.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    });
}
