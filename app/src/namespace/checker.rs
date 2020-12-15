use super::Namespace;

pub fn check(namespace: &Namespace, contents: &str) -> bool {
    let namespace_line = namespace.create_line();

    if !contents.contains(&namespace_line) {
        return false;
    }

    let mut found_namespace = false;
    for line in contents.lines() {
        if line == "" {
            continue;
        }

        if line.starts_with("<?php") || line.starts_with("declare(") {
            if found_namespace {
                return false;
            }
        } else if line == namespace_line {
            if found_namespace {
                return false;
            }
            found_namespace = true;
        } else {
            if !found_namespace {
                return false;
            }
        }
    }

    true
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
    fn correct() {
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
    fn incorrect() {
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

    #[test]
    fn incorrect_position_early() {
        let executable_name = String::from("bin/namespacer");
        let filename = String::from("src/Controller/User/Login.php");
        let base_dir = String::from("src");
        let args = vec![executable_name, filename, base_dir];
        let config = Config::new(&args).unwrap();
        let namespace = Namespace::new(&config);

        let contents = "\
<?php

namespace App\\Controller\\User;

declare(strict_types=1);

class Login {}";

        assert!(!check(&namespace, &contents));
    }

    #[test]
    fn incorrect_position_late() {
        let executable_name = String::from("bin/namespacer");
        let filename = String::from("src/Controller/User/Login.php");
        let base_dir = String::from("src");
        let args = vec![executable_name, filename, base_dir];
        let config = Config::new(&args).unwrap();
        let namespace = Namespace::new(&config);

        let contents = "\
<?php

declare(strict_types=1);

class Login {}

namespace App\\Controller\\User;";

        assert!(!check(&namespace, &contents));
    }
}
