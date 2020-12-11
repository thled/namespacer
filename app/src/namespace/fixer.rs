use super::Namespace;

pub fn fix(namespace: &Namespace, contents: &str) -> String {
    let mut fixed_contents = String::from("");

    let namespace_line = namespace.create_line();
    if !contents.contains("\nnamespace ") {
        if !contents.contains("\ndeclare(") {
            for line in contents.lines() {
                if line.starts_with("<?php") {
                    fixed_contents.push_str(line);
                    fixed_contents.push('\n');
                    fixed_contents.push('\n');
                    fixed_contents.push_str(&namespace_line);
                    println!("no hit");
                } else {
                    fixed_contents.push_str(line);
                }
                fixed_contents.push('\n');
            }
            return fixed_contents;
        } else {
            for line in contents.lines() {
                if line.starts_with("declare(") {
                    fixed_contents.push_str(line);
                    fixed_contents.push('\n');
                    fixed_contents.push('\n');
                    fixed_contents.push_str(&namespace_line);
                    println!("no hit");
                } else {
                    fixed_contents.push_str(line);
                }
                fixed_contents.push('\n');
            }
            return fixed_contents;
        }
    }

    for line in contents.lines() {
        if line.starts_with("namespace ") {
            fixed_contents.push_str(&namespace_line);
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

    fn create_namespace(filename: &str, base_dir: &str) -> Namespace {
        let executable_name = String::from("bin/namespacer");
        let filename = String::from(filename);
        let base_dir = String::from(base_dir);
        let args = vec![executable_name, filename, base_dir];
        let config = Config::new(&args).unwrap();
        Namespace::new(&config)
    }

    #[test]
    fn no_namespace_no_declares() {
        let namespace = create_namespace("src/Controller/Index.php", "src");
        let contents = "\
<?php

class Index {}";

        let fixed_contents = fix(&namespace, contents);

        let expected_result = String::from(
            "\
<?php

namespace App\\Controller;

class Index {}
",
        );
        assert_eq!(fixed_contents, expected_result);
    }

    #[test]
    fn no_namespace_one_declare() {
        let namespace = create_namespace("src/Controller/Index.php", "src");
        let contents = "\
<?php

declare(strict_types=1);

class Index {}";

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
    fn no_namespace_multi_declares() {
        let namespace = create_namespace("src/Controller/Index.php", "src");
        let contents = "\
<?php

declare(strict_types=1);
declare(encoding='UTF-8');

class Index {}";

        let fixed_contents = fix(&namespace, contents);

        let expected_result = String::from(
            "\
<?php

declare(strict_types=1);
declare(encoding='UTF-8');

namespace App\\Controller;

class Index {}
",
        );
        assert_eq!(fixed_contents, expected_result);
    }

    #[test]
    fn correct() {
        let namespace = create_namespace("src/Controller/Index.php", "src");
        let contents = "\
<?php

declare(strict_types=1);

namespace App\\Controller;

class Index {}";

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
    fn incorrect() {
        let namespace = create_namespace("src/Controller/Index.php", "src");
        let contents = "\
<?php

declare(strict_types=1);

namespace App\\Controller\\Incorrect;

class Index {}";

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
}
