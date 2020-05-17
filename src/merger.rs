//todo: Add documentation to all pub structs and functions

use std::error::Error;
use std::path::{Path, PathBuf};
use std::fs;
use filesize;

use super::configuration::conf::Conf;

pub struct Merger {
    conf: Conf
}

impl Merger {
    fn is_allowed(&self, path: &Path) -> bool {
        if !path.is_dir() {
            return true;
        }

        let name = match path.file_name() {
            Some(name) => name,
            _ => return false
        };

        let name = match name.to_str() {
            Some(name) => name,
            _ => return false
        };

        self.conf.is_dir_name_allowed(name)
    }

    fn get_mirror(&self, file: &Path, src: &str, dest: &str) -> Result<PathBuf, Box<dyn Error>> {
        let file = file.to_str()
            .ok_or(format!("Cannot get path for {:?}", file))?;

        let is_source_end_with_slash =
            src.ends_with("/") || src.ends_with("\\");

        let len = if is_source_end_with_slash {
            src.len()
        }
        else {
            src.len() + 1
        };

        let file = &file[len..];

        Ok(PathBuf::from(dest).join(file))
    }

    fn is_files_equal(lft: &Path, rht: &Path) -> std::io::Result<bool> {
        Ok(Self::is_files_have_equal_size(lft, rht)?
            && Self::is_files_have_equal_hash(lft, rht)?)
    }

    fn is_files_have_equal_size(lft: &Path, rht: &Path) -> std::io::Result<bool> {
        let src_size = filesize::file_real_size(lft)?;
        let dst_size = filesize::file_real_size(rht)?;

        Ok(src_size == dst_size)
    }

    fn is_files_have_equal_hash(lft: &Path, rht: &Path) -> std::io::Result<bool> {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let get_hash = |path: &Path| -> std::io::Result<u64> {
            let mut hasher = DefaultHasher::new();

            let content = fs::read(path)?;
            content.hash(&mut hasher);//todo: read file through buffer

            Ok(hasher.finish())
        };

        let source_hash = get_hash(lft)?;
        let dest_hash = get_hash(rht)?;

        Ok(source_hash == dest_hash)
    }
}

impl Merger {
    pub fn new(conf: Conf) -> Merger {
        assert!(conf.is_valid());

        Merger {
            conf
        }
    }

    pub fn start(&self) -> Result<(), Box<dyn Error>>{ //todo:: use Thread pool
        use std::time::Instant;

        let start_time = Instant::now();

        println!("Removing extra files...");
        self.delete_extra_files(self.conf.dest())?;
        println!("Extra files removed\n");

        println!("Merge started with: {}", self.conf);
        self.merge(self.conf.source())?;
        println!("Files merged");

        println!(r#"
-----------------------------------
Merging take: {}s
-----------------------------------
"#, start_time.elapsed().as_secs_f32());

        Ok(())
    }

    fn delete_extra_files(&self, entry: &Path) -> Result<(), Box<dyn Error>>{
        if !self.is_allowed(entry) {
            return Ok(());
        }

        for entry in fs::read_dir(entry)? {
            let entry = entry?;

            if entry.path().is_dir() {
                self.delete_extra_files(entry.path().as_path())?;

                continue;
            }

            assert!(entry.path().is_file());

            let mirror = self.get_mirror(entry.path().as_path(),
                self.conf.dest_as_str(),
                self.conf.source_as_str())?;

            if !mirror.exists() {
                println!("\tExtra file '{}' - removed", entry.path().to_str().unwrap());
                fs::remove_file(entry.path())?;
            }
        }

        Ok(())
    }

    fn merge(&self, entry: &Path) -> Result<(), Box<dyn Error>> {
        if !self.is_allowed(entry) {
            return Ok(());
        }

        for entry in fs::read_dir(entry)? {
            let entry = entry?;

            if entry.path().is_dir() {
                self.merge(entry.path().as_path())?;

                continue;
            }

            assert!(entry.path().is_file());

            if let Err(e) = self.merge_file(entry.path().as_path()) {
                eprintln!("On entry '{:?}' occurs error:\n'{}'", entry, e);
            }
        }

        Ok(())
    }

    fn merge_file(&self, source_file: &Path) -> Result<(), Box<dyn Error>>{
        let dest_file = self.get_mirror(source_file,
                                        self.conf.source_as_str(),
                                        self.conf.dest_as_str())?;

        let dest_str = dest_file.to_str()
            .ok_or(format!("Unexpected internal error: cannot get str from: '{:?}'"
                           , dest_file))?.to_owned();

        if !dest_file.exists() {
            fs::copy(source_file, dest_file)?;
            println!("\tFile '{}' - created", dest_str);
        }
        else if !Self::is_files_equal(source_file, dest_file.as_path())? {
            fs::copy(source_file, dest_file)?;
            println!("\tFile '{}' - replaced", dest_str);
        }

        Ok(())
    }
}

#[cfg(test)]
mod test { //todo: Create tests

}

