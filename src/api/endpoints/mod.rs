pub mod control;

use async_channel::{unbounded, Sender, Receiver};
use std::sync::LazyLock;
pub static CHANNEL: LazyLock<(Sender<Endpoint>, Receiver<Endpoint>)> = LazyLock::new(unbounded);

#[derive(Debug, Clone)]
pub enum Endpoint {
    Control(control::Control),
}

macro_rules! e {
    ($from:path, $to:ident) => {
        impl From<$from> for Endpoint { fn from(value: $from) -> Self { Self::$to(value) } }
    }
}

e!(control::Control, Control);

#[macro_export]
macro_rules! send {
    ($item:expr) => { $crate::api::endpoints::CHANNEL.0.clone().send($item.into()).await.unwrap(); }
}
pub(super) use send;
