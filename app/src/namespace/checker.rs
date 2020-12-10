use super::Namespace;

pub fn check(namespace: &Namespace, contents: &str) -> bool {
    let line = namespace.create_line();

    contents.contains(&line)
}

#[cfg(test)]
mod tests {
    use crate::{namespace::Namespace, Config};

    use super::*;

    #[test]
    fn no_namespace() {
        let executable_name = String::from("bin/namespacer");
        let filename = String::from("src/Controller/User/Login.php");
        let base_dir = String::from("src");
        let args = vec![executable_name, filename, base_dir];
        let config = Config::new(&args).unwrap();
        let namespace = Namespace::new(&config);

        let contents = "\
<?php

declare(strict_types=1);

class Login {}";

        assert!(!check(&namespace, &contents));
    }

    #[test]
    fn correct_namespace() {
        let executable_name = String::from("bin/namespacer");
        let filename = String::from("src/Controller/User/Login.php");
        let base_dir = String::from("src");
        let args = vec![executable_name, filename, base_dir];
        let config = Config::new(&args).unwrap();
        let namespace = Namespace::new(&config);

        let contents = "\
<?php

declare(strict_types=1);

namespace App\\Controller\\User;

class Login {}";

        assert!(check(&namespace, &contents));
    }

    #[test]
    fn incorrect_namespace() {
        let executable_name = String::from("bin/namespacer");
        let filename = String::from("src/Controller/User/Login.php");
        let base_dir = String::from("src");
        let args = vec![executable_name, filename, base_dir];
        let config = Config::new(&args).unwrap();
        let namespace = Namespace::new(&config);

        let contents = "\
<?php

declare(strict_types=1);

namespace App\\Incorrect;

class Login {}";

        assert!(!check(&namespace, &contents));
    }
}
