use std::{error::Error, fs, path::PathBuf};

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
    let contents = fs::read_to_string(&filename)?;

    let fixed_contents = fix_namespace(&filename, &contents);

    let mut tmp_filename = PathBuf::from(&filename);
    tmp_filename.set_extension("ns_tmp");
    fs::write(&tmp_filename, fixed_contents)?;
    fs::rename(&tmp_filename, filename)?;

    Ok(())
}

fn fix_namespace<'a>(filename: &str, contents: &'a str) -> String {
    let mut parts: Vec<&str> = filename.split("/").collect();
    parts.pop();
    let mut ns = String::from("namespace App");
    let mut found_src = false;

    for part in parts {
        if !found_src && part != "src" {
            continue;
        } else if part == "src" {
            found_src = true;
            continue;
        }
        ns.push('\\');
        ns.push_str(part);
    }

    ns.push(';');
    println!("{}", ns);

    let mut fixed_contents = String::from("");
    for line in contents.lines() {
        if line.contains("namespace") {
            fixed_contents.push_str(ns.as_str());
        } else {
            fixed_contents.push_str(line);
        }
        fixed_contents.push('\n');
    }

    fixed_contents
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_namespace() {
        let filename = "src/Controller/Index.php";
        let contents = "\
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
        assert_eq!(fix_namespace(filename, contents), expected_result);
    }

    #[test]
    fn incorrect_namespace() {
        let filename = "src/Controller/Index.php";
        let contents = "\
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
        assert_eq!(fix_namespace(filename, contents), expected_result);
    }

    #[test]
    fn different_filename() {
        let filename = "src/Model/User.php";
        let contents = "\
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
        assert_eq!(fix_namespace(filename, contents), expected_result);
    }

    #[test]
    fn begin_namespace_from_src_dir() {
        let filename = "app/src/Controller/Index.php";
        let contents = "\
<?php

declare(strict_types=1);

namespace App\\Incorrect;

class Index {}";

        let expected_result = "\
<?php

declare(strict_types=1);

namespace App\\Controller;

class Index {}
";
        assert_eq!(fix_namespace(filename, contents), expected_result);
    }
}
