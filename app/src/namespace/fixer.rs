use super::Namespace;

pub fn fix(namespace: &Namespace, contents: &str) -> String {
    let mut fixed_contents = String::from("");
    for line in contents.lines() {
        if line.starts_with("namespace ") {
            fixed_contents.push_str(namespace.create_line().as_str());
        } else {
            fixed_contents.push_str(line);
        }
        fixed_contents.push('\n');
    }

    fixed_contents
}

#[cfg(test)]
mod tests {
    use crate::Config;

    use super::*;

    #[test]
    fn correct_namespace() {
        let executable_name = String::from("bin/namespacer");
        let filename = String::from("src/Controller/Index.php");
        let base_dir = String::from("src");
        let args = vec![executable_name, filename, base_dir];
        let config = Config::new(&args).unwrap();
        let contents = "\
<?php

declare(strict_types=1);

namespace App\\Controller;

class Index {}";

        let namespace = Namespace::new(&config);
        let fixed_contents = fix(&namespace, contents);

        let expected_result = String::from(
            "\
<?php

declare(strict_types=1);

namespace App\\Controller;

class Index {}
",
        );
        assert_eq!(fixed_contents, expected_result);
    }

    #[test]
    fn incorrect_namespace() {
        let executable_name = String::from("bin/namespacer");
        let filename = String::from("src/Controller/Index.php");
        let base_dir = String::from("src");
        let args = vec![executable_name, filename, base_dir];
        let config = Config::new(&args).unwrap();
        let contents = "\
<?php

declare(strict_types=1);

namespace App\\Controller\\Incorrect;

class Index {}";

        let namespace = Namespace::new(&config);
        let fixed_contents = fix(&namespace, contents);

        let expected_result = String::from(
            "\
<?php

declare(strict_types=1);

namespace App\\Controller;

class Index {}
",
        );
        assert_eq!(fixed_contents, expected_result);
    }

    #[test]
    fn different_filename() {
        let executable_name = String::from("bin/namespacer");
        let filename = String::from("src/Entity/User.php");
        let base_dir = String::from("src");
        let args = vec![executable_name, filename, base_dir];
        let config = Config::new(&args).unwrap();
        let contents = "\
<?php

declare(strict_types=1);

namespace App\\Incorrect;

class User {}";

        let namespace = Namespace::new(&config);
        let fixed_contents = fix(&namespace, contents);

        let expected_result = String::from(
            "\
<?php

declare(strict_types=1);

namespace App\\Entity;

class User {}
",
        );
        assert_eq!(fixed_contents, expected_result);
    }

    #[test]
    fn begin_namespace_from_src_dir() {
        let executable_name = String::from("bin/namespacer");
        let filename = String::from("app/src/Controller/Index.php");
        let base_dir = String::from("app/src");
        let args = vec![executable_name, filename, base_dir];
        let config = Config::new(&args).unwrap();
        let contents = "\
<?php

declare(strict_types=1);

namespace App\\Incorrect;

class Index {}";

        let namespace = Namespace::new(&config);
        let fixed_contents = fix(&namespace, contents);

        let expected_result = String::from(
            "\
<?php

declare(strict_types=1);

namespace App\\Controller;

class Index {}
",
        );
        assert_eq!(fixed_contents, expected_result);
    }

    #[test]
    fn multiple_parts() {
        let executable_name = String::from("bin/namespacer");
        let filename = String::from("src/Controller/User/Login.php");
        let base_dir = String::from("src");
        let args = vec![executable_name, filename, base_dir];
        let config = Config::new(&args).unwrap();
        let contents = "\
<?php

declare(strict_types=1);

namespace App\\Incorrect;

class Login {}";

        let namespace = Namespace::new(&config);
        let fixed_contents = fix(&namespace, contents);

        let expected_result = String::from(
            "\
<?php

declare(strict_types=1);

namespace App\\Controller\\User;

class Login {}
",
        );
        assert_eq!(fixed_contents, expected_result);
    }
}
