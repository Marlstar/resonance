use super::macros::post;

post!(resume, Control::Resume);
post!(pause, Control::Pause);

#[derive(Debug, Clone)]
pub enum Control {
    Resume,
    Pause,
}
