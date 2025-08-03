pub mod control;
mod macros;

use async_channel::{unbounded, Sender, Receiver};
use std::sync::LazyLock;
pub static CHANNEL: LazyLock<(Sender<Endpoint>, Receiver<Endpoint>)> = LazyLock::new(unbounded);

#[derive(Debug, Clone)]
pub enum Endpoint {
    Control(control::Control),
}

macro_rules! e { ($from:path, $to:ident) => {
    impl From<$from> for Endpoint { fn from(value: $from) -> Self { Self::$to(value) } }
}}

e!(control::Control, Control);
