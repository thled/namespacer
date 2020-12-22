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
        } else if !found_namespace {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::{namespace::Namespace, Config};

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
    fn no_namespace() {
        let namespace = create_namespace("src/Controller/User/Login.php", "src");

        let contents = "\
<?php

declare(strict_types=1);

class Login {}";

        assert!(!check(&namespace, &contents));
    }

    #[test]
    fn correct() {
        let namespace = create_namespace("src/Controller/User/Login.php", "src");

        let contents = "\
<?php

declare(strict_types=1);

namespace App\\Controller\\User;

class Login {}";

        assert!(check(&namespace, &contents));
    }

    #[test]
    fn incorrect() {
        let namespace = create_namespace("src/Controller/User/Login.php", "src");

        let contents = "\
<?php

declare(strict_types=1);

namespace App\\Incorrect;

class Login {}";

        assert!(!check(&namespace, &contents));
    }

    #[test]
    fn incorrect_position_early() {
        let namespace = create_namespace("src/Controller/User/Login.php", "src");

        let contents = "\
<?php

namespace App\\Controller\\User;

declare(strict_types=1);

class Login {}";

        assert!(!check(&namespace, &contents));
    }

    #[test]
    fn incorrect_position_late() {
        let namespace = create_namespace("src/Controller/User/Login.php", "src");

        let contents = "\
<?php

declare(strict_types=1);

class Login {}

namespace App\\Controller\\User;";

        assert!(!check(&namespace, &contents));
    }
}
