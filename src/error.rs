#[derive(Debug)]
pub enum Error {
    SongNotInstalled,
    SongAlreadyInstalled,
    NoValidRow,

    DatabaseConnection(diesel::ConnectionError),
    DatabaseResult(diesel::result::Error),
    YtDlp(ytdlp_bindings::YtDlpError),

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

use ytdlp_bindings::YtDlpError;
from_error!(YtDlpError, YtDlp);

