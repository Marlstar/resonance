mod impl_from;
use impl_from::e;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    InvalidYTID,
    InvalidYTURL,

    YTDLP(youtube_dl::Error),
    Regex(regex::Error),
    Image(image::ImageError),
    IO(std::io::Error),
    RodioStream(rodio::StreamError),
    RodioPlay(rodio::PlayError),
    RodioDecode(rodio::decoder::DecoderError),
}

e!(youtube_dl::Error, YTDLP);
e!(regex::Error, Regex);
e!(image::ImageError, Image);
e!(std::io::Error, IO);
e!(rodio::StreamError, RodioStream);
e!(rodio::PlayError, RodioPlay);
e!(rodio::decoder::DecoderError, RodioDecode);
