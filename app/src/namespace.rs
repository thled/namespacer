use std::path::PathBuf;

pub mod checker;
pub mod fixer;
use crate::config::Config;

pub struct Namespace {
    vendor: String,
    prefix: String,
    path: PathBuf,
    base_dir: PathBuf,
}

impl Namespace {
    pub fn new(config: &Config) -> Namespace {
        let file_path = PathBuf::from(&config.filename);
        let path = file_path.parent().unwrap();
        let base_dir = PathBuf::from(&config.base_dir);
        Namespace {
            vendor: config.vendor.clone(),
            prefix: config.prefix.clone(),
            path: path.to_path_buf(),
            base_dir,
        }
    }

    fn create_line(&self) -> String {
        let mut line = String::from("namespace ");

        line.push_str(&self.vendor);
        line.push('\\');

        let dir = self.path.strip_prefix(self.base_dir.to_str().unwrap());
        let dir = dir.unwrap().to_str().unwrap();
        let main = dir.replace("/", "\\");
        line.push_str(main.as_str());

        line.push(';');

        line
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_namespace() {}
}
