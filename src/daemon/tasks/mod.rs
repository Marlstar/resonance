pub mod install_deps;

#[macro_export]
macro_rules! run_and_return_message {
    ($fun:path, $msg:expr $(, $opt:expr),*) => {
        use $crate::iced::types::Task;
        async fn _run_() -> Message {
            $fun($($opt,)*).await;
            return $msg;
        }
        return Task::future(_run_());
    }
}
pub use run_and_return_message;

#[macro_export]
macro_rules! run_and_wrap_message {
    ($fun:path, $msg:path, $($opt:expr),*) => {
        use $crate::iced::types::Task;
        async fn _run_() -> Message {
            $msg($fun($($opt,)*))
        }
        return Task::future(_run_());
    }
}
pub use run_and_wrap_message;
