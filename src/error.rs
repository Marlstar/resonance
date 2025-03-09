#[derive(Debug)]
pub enum Error {
    SongNotInstalled,
    SongAlreadyInstalled,
    NoValidRow,

    DatabaseConnection(diesel::ConnectionError),
    DatabaseResult(diesel::result::Error),

    YtDl(youtube_dl::Error),
    YtDlNotSingleVideo,
    YtDlMalformedOutput,

    NoSearchResults,

    IOError(std::io::Error),

    Image(image::ImageError),

    AudioFileRead(std::io::Error),

    BackupFailed(std::io::Error),

    InvalidURL
}

macro_rules! from_error {
    ($f:ident, $s:ident) => {
        impl From<$f> for Error {
            fn from(value: $f) -> Self {
                $s(value)
            }
        }
    }
}
use Error::*;

use diesel::ConnectionError as DieselConnectionError;
from_error!(DieselConnectionError, DatabaseConnection);

use diesel::result::Error as DieselResultError;
from_error!(DieselResultError, DatabaseResult);

use youtube_dl::Error as ytdlError;
from_error!(ytdlError, YtDl);

use std::io::Error as _IOError;
from_error!(_IOError, IOError);

use image::ImageError;
from_error!(ImageError, Image);
