//todo: Add documentation to all pub structs and functions

use std::env;
use std::error::Error;
use std::fs;
use std::fmt;
use std::fmt::{Display, Formatter};

use super::error::ConfError;
use std::path::{PathBuf, Path};

#[derive(Debug, Deserialize, Serialize)]
pub struct Conf {
    from: Box<Path>,
    to: Box<Path>,

    black_list_patterns: Vec<String> //todo: think about reg expr
}

impl Display for Conf {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, r#"
Configuration
    Source: "{}"
    Destination: "{}"
    Exclude: {:?}
"#, self.source_as_str(), self.dest_as_str(), self.black_list_patterns)
    }
}

impl Conf {
    pub fn new(mut args: std::env::Args) -> Result<Conf, Box<dyn Error>> {
        args.next();

        let path_to_json = match args.next() {
            Some(path) => {
                let path = PathBuf::from(path);

                if !path.exists() || !path.is_file() {
                    return Err(ConfError::new("Cannot get path for json"))
                }

                path
            },
            None => {
                let path = env::current_dir()?.join("default.json");

                if !path.exists() || !path.is_file() {
                    return Err(ConfError::new("Cannot get path for default.json"))
                }

                path
            }
        };

        Self::from(path_to_json.as_path())
    }

    pub fn from(json: &Path) -> Result<Conf, Box<dyn Error>> {
        assert!(json.exists());

        let conf: Conf = serde_json::from_str(fs::read_to_string(json)?.as_str())?;

        if conf.is_valid() {
            Ok(conf)
        }
        else {
            Err(ConfError::new("Configuration has invalid folders"))
        }
    }

    pub fn is_valid(&self) -> bool {
        self.from.is_dir() && self.to.is_dir()
    }

    pub fn is_dir_name_allowed(&self, name: &str) -> bool {
        self.black_list_patterns.iter()
            .all(|x| !name.eq(x))
    }
}

/// Implementing getters
impl Conf {
    pub fn source(&self) -> &Path {
        &*self.from
    }
    pub fn source_as_str(&self) -> &str {self.from.to_str().unwrap() }

    pub fn dest(&self) -> &Path {
        &*self.to
    }
    pub fn dest_as_str(&self) -> &str {self.to.to_str().unwrap() }

}

#[cfg(test)]
pub mod test { //todo: Create tests

}