use std::{error::Error, fs, io, path::PathBuf};

struct Namespace {
    vendor: String,
    prefix: String,
    path: PathBuf,
    base_dir: PathBuf,
}

impl Namespace {
    pub fn new(config: &Config) -> Namespace {
        let file_path = PathBuf::from(&config.filename);
        let path = file_path.parent().unwrap();
        let base_dir = PathBuf::from(&config.dir);
        Namespace {
            vendor: config.vendor.clone(),
            prefix: config.prefix.clone(),
            path: path.to_path_buf(),
            base_dir,
        }
    }

    fn create_line(&self) -> String {
        String::from("todo")
    }
}

pub struct Config {
    pub filename: String,
    pub dir: String,
    pub prefix: String,
    pub vendor: String,
}

impl Config {
    pub fn new(args: &Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        let filename = args[1].clone();

        Ok(Config {
            filename,
            dir: String::from("src"),
            prefix: String::from("Pre"),
            vendor: String::from("App"),
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = read_file(&config)?;

    let fixed_contents = fix(&contents, &config);

    write_fix(&fixed_contents, &config)?;

    Ok(())
}

fn read_file(config: &Config) -> Result<String, io::Error> {
    fs::read_to_string(&config.filename)
}

fn write_fix(fixed_contents: &String, config: &Config) -> Result<(), Box<dyn Error>> {
    let filename = &config.filename;
    let mut tmp_filename = PathBuf::from(filename);
    tmp_filename.set_extension("ns_tmp");
    fs::write(&tmp_filename, fixed_contents)?;
    fs::rename(&tmp_filename, filename)?;
    Ok(())
}

fn fix<'a>(contents: &'a str, config: &Config) -> String {
    let namespace = create_namespace(config);

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

fn create_namespace(config: &Config) -> Namespace {
    Namespace::new(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_namespace() {
        let executable_name = String::from("bin/namespacer");
        let filename = String::from("src/Controller/Index.php");
        let args = vec![executable_name, filename];
        let config = Config::new(&args).unwrap();
        let contents = "\
<?php

declare(strict_types=1);

namespace App\\Controller;

class Index {}";

        let fixed_contents = fix(contents, &config);

        let expected_result = String::from(
            "\
<?php

declare(strict_types=1);

todo

class Index {}
",
        );
        assert_eq!(fixed_contents, expected_result);
    }

    #[test]
    fn incorrect_namespace() {
        let executable_name = String::from("bin/namespacer");
        let filename = String::from("src/Controller/Index.php");
        let args = vec![executable_name, filename];
        let config = Config::new(&args).unwrap();
        let contents = "\
<?php

declare(strict_types=1);

namespace App\\Controller\\Incorrect;

class Index {}";

        let fixed_contents = fix(contents, &config);

        let expected_result = String::from(
            "\
<?php

declare(strict_types=1);

todo

class Index {}
",
        );
        assert_eq!(fixed_contents, expected_result);
    }

    #[test]
    fn different_filename() {
        let executable_name = String::from("bin/namespacer");
        let filename = String::from("src/Entity/User.php");
        let args = vec![executable_name, filename];
        let config = Config::new(&args).unwrap();
        let contents = "\
<?php

declare(strict_types=1);

namespace App\\Incorrect;

class Index {}";

        let fixed_contents = fix(contents, &config);

        let expected_result = String::from(
            "\
<?php

declare(strict_types=1);

todo

class Index {}
",
        );
        assert_eq!(fixed_contents, expected_result);
    }

    #[test]
    fn begin_namespace_from_src_dir() {
        let executable_name = String::from("bin/namespacer");
        let filename = String::from("app/src/Controller/Index.php");
        let args = vec![executable_name, filename];
        let config = Config::new(&args).unwrap();
        let contents = "\
<?php

declare(strict_types=1);

namespace App\\Incorrect;

class Index {}";

        let fixed_contents = fix(contents, &config);

        let expected_result = String::from(
            "\
<?php

declare(strict_types=1);

todo

class Index {}
",
        );
        assert_eq!(fixed_contents, expected_result);
    }
}
