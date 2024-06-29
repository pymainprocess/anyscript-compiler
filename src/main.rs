use anyscript_compiler::parser::ReadShebang;
use std::env;

fn main() {
    // Erhalte die Argumente von der Kommandozeile
    let args: Vec<String> = env::args().collect();
    
    // Überprüfe, ob ein Argument übergeben wurde
    if args.len() < 2 {
        eprintln!("Usage: {} <shebang>", args[0]);
        std::process::exit(1);
    }

    // Erhalte das Shebang-Argument
    let shebang = &args[1];
    
    // Erstelle eine neue ReadShebang-Instanz
    let mut parsed = ReadShebang::new(shebang.to_string());
    
    // Drucke den Interpreter und den Pfad
    println!("Interpreter: {:?}", parsed.get_interpreter_line());
    println!("Path: {:?}", parsed.get_path_line());
}