use std::path::{Path, PathBuf};

pub mod checker;
pub mod fixer;
use crate::config::Config;

pub struct Namespace {
    vendor: String,
    prefix: String,
    file_path: PathBuf,
    base_dir: PathBuf,
}

impl Namespace {
    pub fn new(file_path: &Path, config: &Config) -> Namespace {
        let path = file_path.parent().unwrap();
        let base_dir = PathBuf::from(&config.base_dir);
        Namespace {
            vendor: config.vendor.clone(),
            prefix: config.prefix.clone(),
            file_path: path.to_path_buf(),
            base_dir,
        }
    }

    fn create_line(&self) -> String {
        let mut line = String::from("namespace ");

        line.push_str(&self.vendor);

        if !self.prefix.is_empty() {
            line.push('\\');
            line.push_str(&self.prefix);
        }

        let dir = self.file_path.strip_prefix(self.base_dir.to_str().unwrap());
        let dir = dir.unwrap().to_str().unwrap();

        if !dir.is_empty() {
            line.push('\\');

            let main = dir.replace('/', "\\");
            line.push_str(main.as_str());
        }

        line.push(';');

        line
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_namespace(file_path: &str, base_dir: &str) -> Namespace {
        let executable_name = "bin/namespacer";
        let args = vec![
            executable_name.to_owned(),
            file_path.to_owned(),
            base_dir.to_owned(),
        ];
        let config = Config::new(&args).unwrap();
        Namespace::new(&PathBuf::from(&file_path), &config)
    }

    #[test]
    fn no_dirs() {
        let namespace = create_namespace("Kernel.php", "");

        assert_eq!(namespace.create_line(), "namespace App;");
    }

    #[test]
    fn file_dir() {
        let namespace = create_namespace("Controller/Login.php", "");

        assert_eq!(namespace.create_line(), "namespace App\\Controller;");
    }

    #[test]
    fn multi_dir_file() {
        let namespace = create_namespace("Controller/User/Login.php", "");

        assert_eq!(namespace.create_line(), "namespace App\\Controller\\User;");
    }

    #[test]
    fn base_dir() {
        let namespace = create_namespace("src/Controller/User/Login.php", "src");

        assert_eq!(namespace.create_line(), "namespace App\\Controller\\User;");
    }

    #[test]
    fn multi_base_dir() {
        let namespace = create_namespace("app/src/Controller/User/Login.php", "app/src");

        assert_eq!(namespace.create_line(), "namespace App\\Controller\\User;");
    }

    #[test]
    fn vendor() {
        let vendor = "Acme";
        let file_path = "src/Controller/Login.php";
        let args = vec![
            "bin/namespacer".to_owned(),
            file_path.to_owned(),
            "src".to_owned(),
            vendor.to_owned(),
        ];
        let config = Config::new(&args).unwrap();
        let namespace = Namespace::new(&PathBuf::from(&file_path), &config);

        assert_eq!(namespace.create_line(), "namespace Acme\\Controller;");
    }

    #[test]
    fn prefix() {
        let prefix = "Tests";
        let file_path = "tests/Controller/LoginTest.php";
        let args = vec![
            "bin/namespacer".to_owned(),
            file_path.to_owned(),
            "tests".to_owned(),
            "App".to_owned(),
            prefix.to_owned(),
        ];
        let config = Config::new(&args).unwrap();
        let namespace = Namespace::new(&PathBuf::from(&file_path), &config);

        assert_eq!(namespace.create_line(), "namespace App\\Tests\\Controller;");
    }

    #[test]
    fn multi_prefix() {
        let prefix = "Tests\\Unit";
        let file_path = "tests/Controller/LoginTest.php";
        let args = vec![
            "bin/namespacer".to_owned(),
            file_path.to_owned(),
            "tests".to_owned(),
            "App".to_owned(),
            prefix.to_owned(),
        ];
        let config = Config::new(&args).unwrap();
        let namespace = Namespace::new(&PathBuf::from(&file_path), &config);

        assert_eq!(
            namespace.create_line(),
            "namespace App\\Tests\\Unit\\Controller;"
        );
    }
}
