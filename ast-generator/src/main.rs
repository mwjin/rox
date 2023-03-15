use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: ast-generator [output directory]");
        process::exit(64);
    }

    let output_dir = &args[1];
}
