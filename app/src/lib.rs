use std::{error::Error, fs, path::PathBuf};

pub use config::Config;
use namespace::{checker, fixer, Namespace};

mod config;
mod namespace;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let path = PathBuf::from(&config.path);
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let entry_path = entry.path();
            let extension = entry_path.extension().unwrap_or_default();
            if extension == "php" {
                let file_path = entry_path.to_string_lossy().into_owned();
                fix_file(&file_path, &config)?;
            }
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
