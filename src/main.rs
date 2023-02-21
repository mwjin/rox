use std::env;
use std::error::Error;
use std::fs;
use std::io;
use std::io::Write;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Usage: rox [script]");
        process::exit(64);
    } else if args.len() == 2 {
        if let Err(e) = run_file(&args[1]) {
            eprintln!("Application error: {e}");
            process::exit(1);
        };
    } else {
        if let Err(e) = run_prompt() {
            eprintln!("Application error: {e}");
            process::exit(1);
        };
    }
}

fn run_file(path: &String) -> Result<(), Box<dyn Error>> {
    let source = fs::read_to_string(path)?;
    run(&source);
    Ok(())
}

fn run_prompt() -> Result<(), Box<dyn Error>> {
    loop {
        print!("ROX> ");
        io::stdout().flush()?;
        let mut line = String::new();
        let line_size = io::stdin().read_line(&mut line)?;

        // EOF (Ctrl + D)
        if line_size == 0 {
            break;
        }

        run(&line);
    }
    Ok(())
}

fn run(source: &String) {
    println!("Source: {source}");
}
