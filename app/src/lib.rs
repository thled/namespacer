use std::{error::Error, fs};

pub struct Config {
    pub filename: String,
}

impl Config {
    pub fn new(args: &Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        let filename = args[1].clone();

        Ok(Config { filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let filename = config.filename.as_str();
    let content = fs::read_to_string(&filename)?;

    fix_namespace(filename, &content);

    Ok(())
}

fn fix_namespace<'a>(filename: &str, content: &'a str) -> String {
    let mut parts: Vec<&str> = filename.split("/").collect();
    parts.pop();
    let mut ns = String::from("namespace ");
    for part in parts {
        if part == "src" {
            ns.push_str("App");
        } else {
            ns.push('\\');
            ns.push_str(part);
        }
    }
    ns.push(';');
    println!("{}", ns);

    let mut fixed_content = String::from("");
    for line in content.lines() {
        if line.contains("namespace") {
            fixed_content.push_str(ns.as_str());
        } else {
            fixed_content.push_str(line);
        }
        fixed_content.push('\n');
    }

    fixed_content
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_namespace() {
        let filename = "src/Controller/Index.php";
        let content = "\
<?php

declare(strict_types=1);

namespace App\\Controller;

class Index {}";
        let expected_result = String::from(
            "\
<?php

declare(strict_types=1);

namespace App\\Controller;

class Index {}
",
        );
        assert_eq!(fix_namespace(filename, content), expected_result);
    }

    #[test]
    fn incorrect_namespace() {
        let filename = "src/Controller/Index.php";
        let content = "\
<?php

declare(strict_types=1);

namespace App\\Controller\\Incorrect;

class Index {}";

        let expected_result = "\
<?php

declare(strict_types=1);

namespace App\\Controller;

class Index {}
";
        assert_eq!(fix_namespace(filename, content), expected_result);
    }

    #[test]
    fn different_filename() {
        let filename = "src/Model/User.php";
        let content = "\
<?php

declare(strict_types=1);

namespace App\\Incorrect;

class User {}";

        let expected_result = "\
<?php

declare(strict_types=1);

namespace App\\Model;

class User {}
";
        assert_eq!(fix_namespace(filename, content), expected_result);
    }
}
