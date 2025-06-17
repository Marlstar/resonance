mod impl_from;
use impl_from::e;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    InvalidYTID,
    InvalidYTURL,

    YTDLP(youtube_dl::Error),
    Regex(regex::Error),
}

e!(youtube_dl::Error, YTDLP);
e!(regex::Error, Regex);
