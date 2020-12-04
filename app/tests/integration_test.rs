use namespacer;
use std::error::Error;
use std::fs;

#[test]
fn fix_incorrect_namespace() -> Result<(), Box<dyn Error>> {
    let path = "tests/data/src/Controller";
    let filename = format!("{}/{}", path, "incorrect.php");
    let filename = filename.as_str();
    let contents = "\
<?php

declare(strict_types=1);

namespace App\\Controller\\Incorrect;

class Index {}
";
    fs::create_dir_all(path)?;
    fs::write(filename, contents)?;

    let command_name = "/bin/namespacer";
    let args = vec![command_name.to_string(), filename.to_string()];
    let config = namespacer::Config::new(&args)?;

    namespacer::run(config)?;

    let fixed_contents = fs::read_to_string(filename)?;
    let expected_contents = "\
<?php

declare(strict_types=1);

namespace App\\Controller;

class Index {}
";

    assert_eq!(fixed_contents, expected_contents);

    // todo: use catch_unwind to clean up even if failed
    tear_down()?;
    Ok(())
}

fn tear_down() -> Result<(), Box<dyn Error>> {
    fs::remove_dir_all("tests/data/src")?;

    Ok(())
}
