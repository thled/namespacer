use super::Namespace;

pub fn fix(namespace: &Namespace, contents: &str) -> String {
    let namespace_line = namespace.create_line();
    let mut fixed_contents: Vec<&str> = Vec::new();

    let contents_without_ns = remove_namespace(contents);

    let mut has_namespace = false;
    for line in contents_without_ns.iter().rev() {
        if !has_namespace && (line.starts_with("declare(") || line.starts_with("<?php")) {
            fixed_contents.push(&namespace_line);
            fixed_contents.push("");
            has_namespace = true;
        }
        fixed_contents.push(line);
    }

    vec_to_string_rev(fixed_contents)
}

fn remove_namespace(contents: &str) -> Vec<&str> {
    let mut result: Vec<&str> = Vec::new();

    let lines = contents.lines();
    let mut is_prev_line_ns = false;
    for line in lines {
        if is_prev_line_ns && line.eq("") {
            is_prev_line_ns = false;
            continue;
        }

        if line.starts_with("namespace ") {
            is_prev_line_ns = true;
            continue;
        }

        result.push(line)
    }

    result
}

fn vec_to_string_rev(fixed_contents: Vec<&str>) -> String {
    let mut s = String::new();
    for line in fixed_contents.iter().rev() {
        s.push_str(line);
        s.push('\n');
    }
    s
}

#[cfg(test)]
mod tests {
    use crate::Config;

    use super::*;

    fn create_namespace(file_path: &str, base_dir: &str) -> Namespace {
        let executable_name = "bin/namespacer";
        let args = vec![
            executable_name.to_owned(),
            file_path.to_owned(),
            base_dir.to_owned(),
        ];
        let config = Config::new(&args).unwrap();
        Namespace::new(file_path, &config)
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

    #[test]
    fn incorrect_position() {
        let namespace = create_namespace("src/Controller/Index.php", "src");
        let contents = "\
<?php

namespace App\\Controller;

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
}
