#[macro_export]
macro_rules! send {
    ($item:expr) => { $crate::api::endpoints::CHANNEL.0.send($item.into()).await.unwrap(); }
} pub use send;

// Routes

#[macro_export]
macro_rules! post {
    ($name:ident, $item:expr) => {
        pub fn $name() -> axum::routing::MethodRouter {
            axum::routing::post(|| async { $crate::api::endpoints::macros::send!($item); })
        }
    }
} pub use post;
