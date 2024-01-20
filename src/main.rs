use std::env;
use std::path;

struct Cli {
    pattern: String,
    path: path::PathBuf,
}

fn main() {
    let env_args: Vec<String> = env::args().collect();

    let pattern = env_args.get(1).expect("provide pattern");
    let path = env_args.get(2).expect("provide path");

    println!("pattern={:?}; path={:?};", pattern, path);

    let args = Cli {
        pattern: pattern.to_string(),
        path: path::PathBuf::from(path),
    };

    println!("pattern={:?}; path={:?};", args.pattern, args.path);
}
