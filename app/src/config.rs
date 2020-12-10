pub struct Config {
    pub filename: String,
    pub base_dir: String,
    pub prefix: String,
    pub vendor: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let filename = args[1].clone();
        let base_dir = args[2].clone();

        Ok(Config {
            filename,
            base_dir,
            prefix: String::from("Pre"),
            vendor: String::from("App"),
        })
    }
}
