use which::which;

/// Struktur, die eine Shebang-Zeile und die zugehörigen Interpreter- und Pfadzeilen speichert.
pub struct ReadShebang {
    /// Die Shebang-Zeile.
    pub shebang_line: String,
    /// Die Interpreter-Zeile.
    pub interpreter_line: String,
    /// Die Pfad-Zeile.
    pub path_line: String,
}

impl ReadShebang {
    /// Erstellt eine neue `ReadShebang`-Instanz.
    ///
    /// # Argumente
    ///
    /// * `shebang` - Eine Zeichenkette, die die Shebang-Zeile darstellt.
    ///
    /// # Rückgabe
    ///
    /// Gibt eine neue `ReadShebang`-Instanz zurück.
    pub fn new(shebang: String) -> Self {
        Self {
            shebang_line: shebang,
            interpreter_line: String::new(),
            path_line: String::new(),
        }
    }

    /// Gibt die Shebang-Zeile zurück.
    ///
    /// # Rückgabe
    ///
    /// Gibt eine Referenz auf die Shebang-Zeile zurück.
    pub fn get_shebang_line(&self) -> String {
        self.shebang_line.clone()
    }

    /// Gibt die Interpreter-Zeile zurück.
    ///
    /// # Rückgabe
    ///
    /// Gibt eine Kopie der Interpreter-Zeile zurück und speichert sie in `self.interpreter_line`.
    pub fn get_interpreter_line(&mut self) -> String {
        // Erhalte die Shebang-Zeile
        let shebang = self.get_shebang_line();
        // Teile die Shebang-Zeile in Teile
        let parts: Vec<&str> = shebang.split_whitespace().collect();
        // Wenn es mehr als ein Teil gibt, speichere den Interpreter
        if parts.len() > 1 {
            if parts[0] == "#!/usr/bin/env" {
                self.interpreter_line = parts[1].to_string();
            } else {
                let interpreter_path = parts[0];
                let interpreter_parts: Vec<&str> = interpreter_path.split('/').collect();
                if let Some(interpreter) = interpreter_parts.last() {
                    self.interpreter_line = interpreter.to_string();
                }
            }
            self.interpreter_line.clone()
        } else if parts.len() == 1 {
            let interpreter_path = parts[0];
            let interpreter_parts: Vec<&str> = interpreter_path.split('/').collect();
            if let Some(interpreter) = interpreter_parts.last() {
                self.interpreter_line = interpreter.to_string();
            }
            self.interpreter_line.clone()
        } else {
            String::new()
        }
    }

    /// Gibt die Pfad-Zeile zurück.
    ///
    /// # Rückgabe
    ///
    /// Gibt den Pfad zum Interpreter zurück.
    pub fn get_path_line(&mut self) -> String {
        // Erhalte die Interpreter-Zeile
        let interpreter = self.get_interpreter_line();
        // Finde den Pfad des Interpreters
        let path = which(&interpreter).unwrap();
        // Konvertiere den Pfad in einen String
        path.to_str().unwrap().to_string()
    }

    /// Gibt die Pfad-Zeile zurück.
    ///
    /// # Rückgabe
    ///
    /// Gibt eine Referenz auf die Pfad-Zeile zurück.
    pub fn get_path_line_ref(&self) -> &String {
        &self.path_line
    }
}