use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Usage: rox [script]");
        process::exit(64);
    } else if args.len() == 2 {
        if let Err(e) = rox::run_file(&args[1]) {
            eprintln!("Application error: {e}");
            process::exit(1);
        };
    } else {
        if let Err(e) = rox::run_prompt() {
            eprintln!("Application error: {e}");
            process::exit(1);
        };
    }
}
