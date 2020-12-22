use glob::glob;
use std::{error::Error, fs, path::PathBuf};

pub use config::Config;
use namespace::{checker, fixer, Namespace};

mod config;
mod namespace;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let path = PathBuf::from(&config.path);
    if path.is_dir() {
        let php_pattern = "**/*.php";
        let path_with_php_pattern = format!("{}/{}", path.to_str().unwrap(), php_pattern);
        for entry in glob(path_with_php_pattern.as_str())? {
            let entry = entry?;
            let file_path = entry.to_str().unwrap();
            fix_file(&file_path, &config)?;
        }
        Ok(())
    } else {
        fix_file(&config.path, &config)
    }
}

pub fn fix_file(file_path: &str, config: &Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(file_path)?;

    let namespace = Namespace::new(file_path, &config);
    if !checker::check(&namespace, &contents) {
        let fixed_contents = fixer::fix(&namespace, &contents);
        write_fix(&fixed_contents, file_path)?;
    }

    Ok(())
}

fn write_fix(fixed_contents: &str, file_path: &str) -> Result<(), Box<dyn Error>> {
    let mut tmp_filename = PathBuf::from(file_path);
    tmp_filename.set_extension("ns_tmp");
    fs::write(&tmp_filename, fixed_contents)?;
    fs::rename(&tmp_filename, file_path)?;
    Ok(())
}
