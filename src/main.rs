use anyscript_compiler::parser::ReadShebang;
use std::env;

fn main() {
    let args = env::args().nth(1).unwrap();
    let mut parsed = ReadShebang::new(args);
    println!("Interpreter: {:?}", parsed.get_interpreter_line());
    println!("Path: {:?}", parsed.get_path_line());
}