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
    let contents = fs::read_to_string(config.filename)?;

    fix_namespace(&contents);

    Ok(())
}

pub fn fix_namespace(contents: &str) -> &str {
    for line in contents.lines() {
        if line.contains("namespace") {
            println!("Found a namespace!");
        }
    }

    contents
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_fix() {
        let contents = "\
<?php

declare(strict_types=1);

namespace App\\Controller;

class Index {}";

        assert_eq!(contents, fix_namespace(contents));
    }
}
