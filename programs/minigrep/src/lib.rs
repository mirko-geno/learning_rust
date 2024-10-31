use std::{fs, error::Error};


pub struct Config {
    pub query: String,
    pub file_path: String
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        // Since &args[0] is the location of the binary we start taking
        // values from &args[1]
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        Ok(Config { query, file_path })
    }
}


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(&config.file_path)?;
    println!("With text:\n{content}");
    Ok(())
}
