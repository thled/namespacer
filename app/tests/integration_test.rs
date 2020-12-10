use namespacer;
use std::fs;
use std::{error::Error, io};

#[test]
fn fix_incorrect_namespace() -> Result<(), Box<dyn Error>> {
    let base_dir = "tests/data/src";
    let path = format!("{}/{}", base_dir, "Controller/User");
    let filename = format!("{}/{}", path, "Login.php");
    let contents = "\
<?php

declare(strict_types=1);

namespace App\\Controller\\Incorrect;

class Login {}
";
    fs::create_dir_all(&path)?;
    fs::write(&filename, &contents)?;

    let executable_name = String::from("/bin/namespacer");
    let args = vec![executable_name, filename.clone(), base_dir.to_string()];
    let config = namespacer::Config::new(&args)?;

    namespacer::run(config)?;

    let fixed_contents = fs::read_to_string(&filename)?;
    let expected_contents = "\
<?php

declare(strict_types=1);

namespace App\\Controller\\User;

class Login {}
";

    assert_eq!(fixed_contents, expected_contents);

    teardown(&base_dir)?;
    Ok(())
}

fn teardown(dir: &str) -> Result<(), io::Error> {
    fs::remove_dir_all(dir)
}
