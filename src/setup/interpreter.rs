use which::which;

/// Structure that stores a shebang line and associated interpreter and path lines.
///
/// ```rust
/// pub struct ReadShebang {
///     /// The shebang line.
///     pub shebang_line: String,
///     /// The interpreter line.
///     pub interpreter_line: String,
///     /// The path line.
///     pub path_line: String,
/// }
/// ```
pub struct ReadShebang {
    /// The shebang line.
    pub shebang_line: String,
    /// The interpreter line.
    pub interpreter_line: String,
    /// The path line.
    pub path_line: String,
}

impl ReadShebang {
    /// Creates a new `ReadShebang` instance.
    ///
    /// # Arguments
    ///
    /// * `shebang` - A string representing the shebang line.
    ///
    /// # Returns
    ///
    /// Returns a new `ReadShebang` instance.
    ///
    /// ```rust
    /// pub fn new(shebang: String) -> Self {
    ///     Self {
    ///         shebang_line: shebang,
    ///         interpreter_line: String::new(),
    ///         path_line: String::new(),
    ///     }
    /// }
    /// ```
    pub fn new(shebang: String) -> Self {
        Self {
            shebang_line: shebang,
            interpreter_line: String::new(),
            path_line: String::new(),
        }
    }

    /// Returns the shebang line.
    ///
    /// # Returns
    ///
    /// Returns a reference to the shebang line.
    ///
    /// ```rust
    /// pub fn get_shebang_line(&self) -> String {
    ///     let _shebang = self.shebang_line.clone();
    ///     _shebang
    /// }
    /// ```
    pub fn get_shebang_line(&self) -> String {
        let _shebang = self.shebang_line.clone();
        _shebang
    }

    /// Returns the interpreter line.
    ///
    /// # Returns
    ///
    /// Returns a copy of the interpreter line and stores it in `self.interpreter_line`.
    ///
    /// ```rust
    /// pub fn get_interpreter_line(&mut self) -> String {
    ///     let shebang = self.get_shebang_line();
    ///     let parts: Vec<&str> = shebang.split_whitespace().collect();
    ///     if parts.len() > 1 {
    ///         self.interpreter_line = parts[1].to_string();
    ///         self.interpreter_line.clone()
    ///     } else {
    ///         String::new()
    ///     }
    /// }
    /// ```
    pub fn get_interpreter_line(&mut self) -> String {
        // Erhalte die Shebang Zeile
        let shebang = self.get_shebang_line();
        // Teile die Shebang Zeile in Teile
        let parts: Vec<&str> = shebang.split_whitespace().collect();
        // Wenn es mehr als ein Teil gibt, speichere den Interpreter
        if parts.len() > 1 {
            self.interpreter_line = parts[1].to_string();
            self.interpreter_line.clone()
        } else {
            String::new()
        }
    }

    /// Returns the path line.
    ///
    /// # Returns
    ///
    /// Returns the path to the interpreter.
    ///
    /// ```rust
    /// pub fn get_path_line(&mut self) -> String {
    ///     let interpreter = self.get_interpreter_line();
    ///     let path = which(&interpreter).unwrap();
    ///     path.to_str().unwrap().to_string()
    /// }
    /// ```
    pub fn get_path_line(&mut self) -> String {
        // Erhalte die Interpreter-Zeile
        let interpreter = self.get_interpreter_line();
        // Finde den Pfad des Interpreters
        let path = which(&interpreter).unwrap();
        // Konvertiere den Pfad in einen String
        path.to_str().unwrap().to_string()
    }

    /// Returns the path line.
    ///
    /// # Returns
    ///
    /// Returns a reference to the path line.
    ///
    /// ```rust
    /// pub fn get_path_line_ref(&self) -> &String {
    ///     &self.path_line
    /// }
    /// ```
    pub fn get_path_line_ref(&self) -> &String {
        &self.path_line
    }
}