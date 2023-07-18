use serial_test::serial;
use std::fs;
use std::{error::Error, io};

use namespacer;

#[test]
#[serial]
fn fix_file() -> Result<(), Box<dyn Error>> {
    let file_path = create_login_file()?;
    let config = get_config(file_path.as_str())?;

    namespacer::run(config)?;

    assert_login_file()?;

    teardown()?;
    Ok(())
}

#[test]
#[serial]
fn fix_dir() -> Result<(), Box<dyn Error>> {
    create_login_file()?;
    create_logout_file()?;
    let dir_path = format!("{}/{}", get_base_dir(), "Controller/User");
    let config = get_config(dir_path.as_str())?;

    namespacer::run(config)?;

    assert_login_file()?;
    assert_logout_file()?;

    teardown()?;
    Ok(())
}

#[test]
#[serial]
fn fix_dir_recursively() -> Result<(), Box<dyn Error>> {
    create_login_file()?;
    create_index_file()?;
    let config = get_config(get_base_dir().as_str())?;

    namespacer::run(config)?;

    assert_login_file()?;
    assert_index_file()?;

    teardown()?;
    Ok(())
}

fn get_base_dir() -> String {
    String::from("tests/data/src")
}

fn teardown() -> Result<(), io::Error> {
    fs::remove_dir_all(get_base_dir())
}

fn get_config(path: &str) -> Result<namespacer::Config, &'static str> {
    let executable_name = "/bin/namespacer";
    let vendor = "Acme";
    let prefix = "Foo";
    let args = vec![
        executable_name.to_owned(),
        path.to_owned(),
        get_base_dir(),
        vendor.to_owned(),
        prefix.to_owned(),
    ];
    namespacer::Config::new(&args)
}

fn create_login_file() -> Result<String, Box<dyn Error>> {
    let path = format!("{}/{}", get_base_dir(), "Controller/User");
    fs::create_dir_all(&path)?;
    let file_path = format!("{}/{}", path, "Login.php");
    let contents = "\
<?php

declare(strict_types=1);

namespace App\\Controller\\Incorrect;

class Login {}
";
    fs::write(&file_path, &contents)?;
    Ok(file_path)
}

fn assert_login_file() -> Result<(), Box<dyn Error>> {
    let path = format!("{}/{}", get_base_dir(), "Controller/User");
    let file_path = format!("{}/{}", path, "Login.php");
    let fixed_contents = fs::read_to_string(&file_path)?;
    let expected_contents = "\
<?php

declare(strict_types=1);

namespace Acme\\Foo\\Controller\\User;

class Login {}
";

    assert_eq!(fixed_contents, expected_contents);
    Ok(())
}

fn create_logout_file() -> Result<String, Box<dyn Error>> {
    let path = format!("{}/{}", get_base_dir(), "Controller/User");
    fs::create_dir_all(&path)?;
    let file_path = format!("{}/{}", path, "Logout.php");
    let contents = "\
<?php

namespace Acme\\Foo\\Controller\\User;

declare(strict_types=1);

class Logout {}
";
    fs::write(&file_path, &contents)?;
    Ok(file_path)
}

fn assert_logout_file() -> Result<(), Box<dyn Error>> {
    let path = format!("{}/{}", get_base_dir(), "Controller/User");
    let file_path = format!("{}/{}", path, "Logout.php");
    let fixed_contents = fs::read_to_string(&file_path)?;
    let expected_contents = "\
<?php

declare(strict_types=1);

namespace Acme\\Foo\\Controller\\User;

class Logout {}
";

    assert_eq!(fixed_contents, expected_contents);
    Ok(())
}

fn create_index_file() -> Result<String, Box<dyn Error>> {
    let path = format!("{}/{}", get_base_dir(), "Controller");
    fs::create_dir_all(&path)?;
    let file_path = format!("{}/{}", path, "Index.php");
    let contents = "\
<?php

declare(strict_types=1);

namespace Acme\\Foo\\Controller\\User;

class Index {}
";
    fs::write(&file_path, &contents)?;
    Ok(file_path)
}

fn assert_index_file() -> Result<(), Box<dyn Error>> {
    let path = format!("{}/{}", get_base_dir(), "Controller");
    let file_path = format!("{}/{}", path, "Index.php");
    let fixed_contents = fs::read_to_string(&file_path)?;
    let expected_contents = "\
<?php

declare(strict_types=1);

namespace Acme\\Foo\\Controller;

class Index {}
";

    assert_eq!(fixed_contents, expected_contents);
    Ok(())
}
