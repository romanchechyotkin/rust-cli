pub struct Config {
    pub pattern: String,
    pub path: String,
}

impl Config {
    pub fn new(args: &Vec<String>) -> Config {
        let pattern = args.get(1).expect("provide pattern").clone();
        let path = args.get(2).expect("provide path").clone();

        Config { pattern, path }
    }
}
