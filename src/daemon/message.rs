#[derive(Debug, Clone)]
pub enum Message {
    FFmpegDownloaded,
    YtDlpDownloaded,

    Tray(tray_icon::menu::MenuEvent),
}
