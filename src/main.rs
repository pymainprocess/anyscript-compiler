use anyscript_compiler::parser::ReadShebang;
use std::env;

fn read_shebang(shebang: &str) -> (String, String) {
    let mut parsed = ReadShebang::new(shebang.to_string());
    (parsed.get_interpreter_line(), parsed.get_path_line())
}

fn show_usage(stdout: bool) {
    if stdout {
        println!("Usage: anyscript [OPTION] <FLAGS>");
        println!("   -s, --shebang [Shebang Line]             Control the Shebang.");
        println!("   -h, --help                              Show this help message.");
    } else {
        eprintln!("Usage: anyscript [OPTION] <FLAGS>");
        eprintln!("   -s, --shebang [Shebang Line]             Control the Shebang.");
        eprintln!("   -h, --help                              Show this help message.");
    }
}

fn main() {
    let all_args: Vec<String> = env::args().collect();
    if all_args.len() < 2 {
        show_usage(true);
        return;
    }

    let arg_1 = all_args.get(1).unwrap();
    if arg_1 == "--shebang" || arg_1 == "-s" {
        if all_args.len() < 3 {
            eprintln!("Fehler: Shebang-Zeile fehlt.");
            show_usage(false);
            return;
        }
        let arg_2 = all_args.get(2).unwrap();
        let (interpreter, path) = read_shebang(&arg_2);
        println!("Interpreter: {}", interpreter);
        println!("Path: {}", path);
    } else if arg_1 == "--help" || arg_1 == "-h" {
        show_usage(false);
    } else {
        show_usage(true);
    }
}