pub struct Config {
    pub filename: String,
    pub base_dir: String,
    pub vendor: String,
    pub prefix: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let filename = args[1].clone();
        let base_dir = args[2].clone();
        let vendor = match args.get(3) {
            Some(arg) => arg,
            None => "App",
        };
        let prefix = match args.get(4) {
            Some(arg) => arg,
            None => "",
        };

        Ok(Config {
            filename,
            base_dir,
            vendor: vendor.to_string(),
            prefix: prefix.to_string(),
        })
    }
}
