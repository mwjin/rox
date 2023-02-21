use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("Usage: rox [script]");
        process::exit(64);
    } else if args.len() == 2 {
        // TODO: Implement runFile
        println!("Run the file");
    } else {
        // TODO: Implement runPrompt
        println!("Run the prompt");
    }
}
