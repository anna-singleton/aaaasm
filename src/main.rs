mod interpreter;
mod parser;

use interpreter::Interpreter;

fn main() {
    let input = include_str!("test.aaaasm");

    let parsed_input = parser::parse_code(input);

    if parsed_input.is_err() {
        eprintln!("fatal error: couldnt parse code, exiting");
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
        Ok(result) => println!("program finished with {} in acc", result),
        Err(err) => println!("program failed with error: {}", err),
    }
}
