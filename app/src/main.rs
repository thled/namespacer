use std::{env, process};

const HELP: &str = "\
namespacer 0.1.3
Thomas Le Duc <dev@tleduc.de>

namespacer automatically fixes wrong namespace declarations in PHP files.
It does so in regards to the standards PSR-4 and PSR-12.

Project home page: https://github.com/thled/namespacer


USAGE:
    namespacer PATH BASE_DIR [VENDOR [PREFIX]]

PATH:
    - Relative path to the PHP file to fix.
    - Relative path to a directory containing PHP files to fix (recursivly traverses subdirectories).

BASE_DIR:
    Prefix of the PATH which will be ignored constructing the namespace.

VENDOR:
    The top-level namespace name (default: App).

PREFIX:
    Additional namespace names which follow after the VENDOR.

EXAMPLES:
    ./namespacer src/Controller/Login.php src
    ./namespacer app/src/Controller/Login.php app/src
    ./namespacer tests/Unit/LoginTest.php tests App Tests
    ./namespacer src/ src
    ./namespacer tests/ tests App Tests
";

fn main() {
    let args: Vec<String> = env::args().collect();

    let first_arg = match args.get(1) {
        Some(arg) => arg,
        _ => "",
    };
    if first_arg == "--help" {
        println!("{}", HELP);
        return;
    }

    let config = namespacer::Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = namespacer::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
