use std::env;
use std::fs;
 
mod config; 

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env_args: Vec<String> = env::args().collect();

    let cfg = config::Config::new(&env_args);
    println!("pattern={:?}; path={:?};", cfg.pattern, cfg.path);

    let result = fs::read_to_string(cfg.path);
    let content = match result {
        Ok(content) => content,
        Err(error) => {
            return Err(error.into());
        }
    };

    let mut counter = 0;
    for line in content.lines() {
        if line.contains(&cfg.pattern) {
            counter += 1;
            println!("{counter}. {line}");
        }
    }

    Ok(())
}
