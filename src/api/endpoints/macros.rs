#[macro_export]
macro_rules! send {
    ($item:expr) => { $crate::api::endpoints::CHANNEL.0.clone().send($item.into()).await.unwrap(); }
}
pub(super) use send;
