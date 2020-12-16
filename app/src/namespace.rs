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

        if self.prefix != "" {
            line.push('\\');
            line.push_str(&self.prefix);
        }

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

    fn create_namespace(filename: &str, base_dir: &str) -> Namespace {
        let executable_name = String::from("bin/namespacer");
        let filename = String::from(filename);
        let base_dir = String::from(base_dir);
        let args = vec![executable_name, filename, base_dir];
        let config = Config::new(&args).unwrap();
        Namespace::new(&config)
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
        let args = vec![
            "bin/namespacer".to_owned(),
            "src/Controller/Login.php".to_owned(),
            "src".to_owned(),
            vendor.to_owned(),
        ];
        let config = Config::new(&args).unwrap();
        let namespace = Namespace::new(&config);

        assert_eq!(namespace.create_line(), "namespace Acme\\Controller;");
    }

    #[test]
    fn prefix() {
        let prefix = "Tests";
        let args = vec![
            "bin/namespacer".to_owned(),
            "tests/Controller/LoginTest.php".to_owned(),
            "tests".to_owned(),
            "App".to_owned(),
            prefix.to_owned(),
        ];
        let config = Config::new(&args).unwrap();
        let namespace = Namespace::new(&config);

        assert_eq!(namespace.create_line(), "namespace App\\Tests\\Controller;");
    }
}
