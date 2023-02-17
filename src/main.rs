mod interpreter;
mod parser;
mod cli;
use clap::Parser;

use interpreter::Interpreter;

fn main() {
    let cli = cli::CLI::parse();

    match cli.command {
        cli::Commands::Run {file, ..} => run(file),
    };
}

fn run(file: String) {
    let maybe_input = std::fs::read_to_string(file);
    let input = match maybe_input {
        Ok(s) => s,
        Err(err) => {eprintln!("Could not read file: {}", err); return},
    };

    let parsed_input = parser::parse_code(&input);

    if parsed_input.is_err() {
        eprintln!("fatal error: couldnt parse code, error: \n{},\nexiting",
                  parsed_input.unwrap_err());
        return
    }


    let instructions = match parsed_input {
        Ok(instructions) => instructions,
        Err(err) => {
            eprintln!("Couldn't parse instructions, error: {}", err);
            return;
        },
    };

    let mut interpreter = Interpreter::new(instructions);

    match interpreter.run_program() {
        Ok(result) => println!("program finished with {} in the accumulator",
                               result),
        Err(err) => println!("program failed with error: {}", err),
    }
}
