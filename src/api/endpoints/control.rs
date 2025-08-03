use axum::routing::{post, MethodRouter};

pub fn resume() -> MethodRouter {
    post(|| async { super::send!(Control::Resume); })
}
pub fn pause() -> MethodRouter {
    post(|| async { super::send!(Control::Pause); })
}

#[derive(Debug, Clone)]
pub enum Control {
    Resume,
    Pause,
}
