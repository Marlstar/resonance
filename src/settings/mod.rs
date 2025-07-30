use std::fs::File;
use std::io::Read;

#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone, PartialEq)]
pub struct Settings {
    pub start_minimised: bool,
}

impl Settings {
    pub fn load_or_default() -> Self {
        match Self::load() {
            Ok(a) => a,
            Err(e) => {
                println!("[settings] error loading from file, reverting to default settings ({e:?})");
                Self::default()
            }
        }
    }

    pub fn load() -> Result<Self, SettingsError> {
        let dir = &*crate::dirs::SETTINGS;
        let mut buf: Vec<u8> = vec![];
        let mut file = File::open(dir)?;
        file.read_to_end(&mut buf)?;
        let settings = ron::de::from_bytes::<Settings>(&buf)?;
        return Ok(settings);
    }


    pub fn save(&self) {
        match self.write() {
            Ok(_) => println!("[settings] saved successfully"),
            Err(e) => println!("[settings] error saving settings ({e:?})"),
        }
    }

    fn write(&self) -> Result<(), SettingsError> {
        let bytes = ron::to_string(self)?
            .into_bytes();
        std::fs::write(&*crate::dirs::SETTINGS, &bytes)?;
        Ok(())
    }
}

#[allow(clippy::derivable_impls)]
impl Default for Settings {
    fn default() -> Self {
        Self {
            start_minimised: false,
        }
    }
}

#[derive(Debug)]
pub enum SettingsError {
    IO(std::io::Error),
    Serialise(ron::Error),
    Deserialise(ron::de::SpannedError),
}
impl From<std::io::Error> for SettingsError {
    fn from(value: std::io::Error) -> Self { Self::IO(value) }
}
impl From<ron::Error> for SettingsError {
    fn from(value: ron::Error) -> Self { Self::Serialise(value) }
}
impl From<ron::de::SpannedError> for SettingsError {
    fn from(value: ron::de::SpannedError) -> Self { Self::Deserialise(value) }
}
