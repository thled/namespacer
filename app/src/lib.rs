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

fn fix_namespace<'a>(_filename: &str, content: &'a str) -> String {
    let mut fixed_content = String::from("");
    for line in content.lines() {
        if line.contains("namespace") {
            let fixed_namespace = "namespace App\\Controller;";
            fixed_content.push_str(fixed_namespace);
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
}
