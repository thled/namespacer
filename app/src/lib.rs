use std::{error::Error, fs, io, path::PathBuf};

pub use config::Config;
use namespace::{checker, fixer, Namespace};

mod config;
mod namespace;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = read_file(&config)?;

    let namespace = Namespace::new(&config);
    if !checker::check(&namespace, &contents) {
        let fixed_contents = fixer::fix(&namespace, &contents);
        write_fix(&fixed_contents, &config)?;
    }

    Ok(())
}

fn read_file(config: &Config) -> Result<String, io::Error> {
    fs::read_to_string(&config.filename)
}

fn write_fix(fixed_contents: &str, config: &Config) -> Result<(), Box<dyn Error>> {
    let filename = &config.filename;
    let mut tmp_filename = PathBuf::from(filename);
    tmp_filename.set_extension("ns_tmp");
    fs::write(&tmp_filename, fixed_contents)?;
    fs::rename(&tmp_filename, filename)?;
    Ok(())
}
