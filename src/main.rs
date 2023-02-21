use std::env;
use std::error::Error;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("Usage: rox [script]");
        process::exit(64);
    } else if args.len() == 2 {
        run_file(&args[1]);
    } else {
        // TODO: Implement runPrompt
        println!("Run the prompt");
    }
}

fn run_file(path: &String) -> Result<(), Box<dyn Error>> {
    let source = fs::read_to_string(path)?;
    run(&source);
    Ok(())
}

fn run(source: &String) {
    println!("Source: {source}");
}
