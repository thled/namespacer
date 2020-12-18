use serial_test::serial;
use std::fs;
use std::{error::Error, io};

use namespacer;

#[test]
#[serial]
fn fix_file() -> Result<(), Box<dyn Error>> {
    let base_dir = "tests/data/src";
    let path = format!("{}/{}", base_dir, "Controller/User");
    let file_path = format!("{}/{}", path, "Login.php");
    let contents = "\
<?php

declare(strict_types=1);

namespace App\\Controller\\Incorrect;

class Login {}
";
    fs::create_dir_all(&path)?;
    fs::write(&file_path, &contents)?;

    let executable_name = "/bin/namespacer";
    let vendor = "Acme";
    let prefix = "Foo";
    let args = vec![
        executable_name.to_owned(),
        file_path.clone(),
        base_dir.to_owned(),
        vendor.to_owned(),
        prefix.to_owned(),
    ];
    let config = namespacer::Config::new(&args)?;

    namespacer::run(config)?;

    let fixed_contents = fs::read_to_string(&file_path)?;
    let expected_contents = "\
<?php

declare(strict_types=1);

namespace Acme\\Foo\\Controller\\User;

class Login {}
";

    assert_eq!(fixed_contents, expected_contents);

    teardown(&base_dir)?;
    Ok(())
}

#[test]
#[serial]
fn fix_dir() -> Result<(), Box<dyn Error>> {
    let base_dir = "tests/data/src";
    let path = format!("{}/{}", base_dir, "Controller/User");
    fs::create_dir_all(&path)?;

    let file_path_login = format!("{}/{}", path, "Login.php");
    let contents_login = "\
<?php

declare(strict_types=1);

namespace App\\Controller\\Incorrect;

class Login {}
";
    fs::write(&file_path_login, &contents_login)?;

    let file_path_logout = format!("{}/{}", path, "Logout.php");
    let contents_logout = "\
<?php

namespace App\\Controller\\User;

declare(strict_types=1);

class Logout {}
";
    fs::write(&file_path_logout, &contents_logout)?;

    let executable_name = "/bin/namespacer";
    let vendor = "Acme";
    let prefix = "Foo";
    let args = vec![
        executable_name.to_owned(),
        path.clone(),
        base_dir.to_owned(),
        vendor.to_owned(),
        prefix.to_owned(),
    ];
    let config = namespacer::Config::new(&args)?;

    namespacer::run(config)?;

    let fixed_contents_login = fs::read_to_string(&file_path_login)?;
    let expected_contents_login = "\
<?php

declare(strict_types=1);

namespace Acme\\Foo\\Controller\\User;

class Login {}
";

    assert_eq!(fixed_contents_login, expected_contents_login);

    let fixed_contents_logout = fs::read_to_string(&file_path_logout)?;
    let expected_contents_logout = "\
<?php

declare(strict_types=1);

namespace Acme\\Foo\\Controller\\User;

class Logout {}
";

    assert_eq!(fixed_contents_logout, expected_contents_logout);

    teardown(&base_dir)?;
    Ok(())
}

fn teardown(dir: &str) -> Result<(), io::Error> {
    fs::remove_dir_all(dir)
}
