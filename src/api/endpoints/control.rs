use axum::routing::{post, MethodRouter};
use super::macros::send;

pub fn resume() -> MethodRouter {
    post(|| async { send!(Control::Resume); })
}
pub fn pause() -> MethodRouter {
    post(|| async { send!(Control::Pause); })
}

#[derive(Debug, Clone)]
pub enum Control {
    Resume,
    Pause,
}
