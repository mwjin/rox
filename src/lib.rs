use std::error::Error;
use std::fs;
use std::io::{self, Write};

pub fn run_file(path: &String) -> Result<(), Box<dyn Error>> {
    let source = fs::read_to_string(path)?;
    run(&source);
    Ok(())
}

pub fn run_prompt() -> Result<(), Box<dyn Error>> {
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
