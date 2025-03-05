use std::path::Path;
use lofty::config::ParseOptions;
use lofty::file::AudioFile;
use lofty::mpeg::MpegProperties;
use lofty::mpeg::MpegFile;
use std::fs::File;
use crate::Error;

pub fn get_mp3_metadata(path: impl AsRef<Path>) -> Result<MpegProperties, Error> {
    let mut file = match File::open(path) {
        Ok(a) => a,
        Err(e) => return Err(Error::AudioFileRead(e))
    };

    let mp3 = MpegFile::read_from(&mut file, ParseOptions::new())?;
    return Ok(mp3.properties().to_owned());
}
