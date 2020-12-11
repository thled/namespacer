use std::path::{Path, PathBuf};

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

        let dir = self.path.strip_prefix(self.base_dir.to_str().unwrap());
        let dir = dir.unwrap().to_str().unwrap();

        if dir != "" {
            line.push('\\');

            let main = dir.replace("/", "\\");
            line.push_str(main.as_str());
        }

        line.push(';');

        line
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_dirs() {
        let executable_name = String::from("bin/namespacer");
        let filename = String::from("Kernel.php");
        let base_dir = String::from("");
        let args = vec![executable_name, filename, base_dir];
        let config = Config::new(&args).unwrap();
        let namespace = Namespace::new(&config);

        let line = namespace.create_line();

        assert_eq!(line, "namespace App;");
    }

    #[test]
    fn no_base_dir() {
        let executable_name = String::from("bin/namespacer");
        let filename = String::from("Controller/User/Login.php");
        let base_dir = String::from("");
        let args = vec![executable_name, filename, base_dir];
        let config = Config::new(&args).unwrap();
        let namespace = Namespace::new(&config);

        let line = namespace.create_line();

        assert_eq!(line, "namespace App\\Controller\\User;");
    }

    #[test]
    fn simple_base_dir() {
        let executable_name = String::from("bin/namespacer");
        let filename = String::from("src/Controller/User/Login.php");
        let base_dir = String::from("src");
        let args = vec![executable_name, filename, base_dir];
        let config = Config::new(&args).unwrap();
        let namespace = Namespace::new(&config);

        let line = namespace.create_line();

        assert_eq!(line, "namespace App\\Controller\\User;");
    }

    #[test]
    fn multi_base_dir() {
        let executable_name = String::from("bin/namespacer");
        let filename = String::from("app/src/Controller/User/Login.php");
        let base_dir = String::from("app/src");
        let args = vec![executable_name, filename, base_dir];
        let config = Config::new(&args).unwrap();
        let namespace = Namespace::new(&config);

        let line = namespace.create_line();

        assert_eq!(line, "namespace App\\Controller\\User;");
    }
}
