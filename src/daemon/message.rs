use std::sync::Arc;
type ArcResult<T> = Arc<crate::Result<T>>;

#[derive(Debug, Clone)]
pub enum Message {
    FFmpegDownloaded,
    YtDlpDownloaded,

    Tray(tray_icon::menu::MenuEvent),
}
