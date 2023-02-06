mod interpreter;
mod parser;

use interpreter::Interpreter;

fn main() {
    let input = include_str!("test.aaaasm");

    let parsed_input = parser::parse_code(input);

    println!("parsed input:\n{:?}", parsed_input);

    if parsed_input.is_err() {
        eprintln!("fatal error: couldnt parse code, exiting");
    }

    let instructions = parsed_input.unwrap();
}
