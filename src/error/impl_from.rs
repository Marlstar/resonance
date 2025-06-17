#[macro_export]
macro_rules! e {
    ($from:path, $to:ident) => {
        impl From<$from> for Error { fn from(value: $from) -> Self { Self::$to(value) } }
    }
}

pub use e;
