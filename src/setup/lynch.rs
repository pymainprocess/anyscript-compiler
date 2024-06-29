use std::ffi::CString;
use std::path::{Path, PathBuf};
use std::fs::{File, write};
use std::io::{BufReader, BufWriter, Read};

struct LynchReader {
    filepath: String,
    content: String,
    result: String,
}

impl LynchReader {

    fn new(filepath: String) -> Self {
        LynchReader {
            filepath,
            content: String::new(),
            result: String::new(),
        }
    }

    fn from_cstring(filepath: CString) -> Self {
        let _string = filepath.to_str().unwrap().to_string();
        Self {
            filepath: _string.clone(),
            content: String::new(),
            result: String::new(),
        }
    }

    fn content_to_string(&mut self) -> String {
        let _file = File::open(&self.filepath).expect("Could not open file.");
        let mut _content = String::new();
        let mut reader = BufReader::new(_file);
        reader.read_to_string(&mut _content).expect("Could not read file.");
        _content
    }

    fn content_to_cstring(&mut self) -> CString {
        let _content = self.content_to_string();
        CString::new(_content).expect("Could not convert string to cstring.")
    }

    fn read_content(&mut self) {
        let _content = self.content_to_string();
        self.content = _content;
    }
    
    fn decode_lynch(&mut self) {
        let _content = self.content_to_string();
        let mut _result = String::new();
        // Wir schreiben den Code um und codieren es in Zahlen um
        // Das sieht so aus, dass jede Zahl die Stelle des Alphabets entspricht mit dem Splitter 0
        // 0 steht für die Trennung zwischen den verschiedenen Zeilen
        // 1 steht für a
        // 2 steht für b
        // usw...
        // Sonderzeichen bleiben wo sie sind, das Resultat ist ein Code-String mit dem wir arbeiten
        // Es gibt 26, 1 bis 26 sind Buchstaben,
        // vor und nach Zahlen kommen 00, 0 steht für nächster Buchstabe, aber 00 steht nur dafür, dass es eine Zahl und kein Buchstabe ist
        // Alle Zahlen nach 27 sind ungültig, heißt dann ist der String Invalid, der String dient dazu, den String zu codieren, also besser für das Programm hier
        // Jetzt müssen wir den Content so splitten und speichern, dass es in einem Vektor ist, wo immer exakt ein Zeichen drinnen ist.
        let mut _liste = Vec::new();
        for _line in _content.split('\n') {
            for _char in _line.chars() {
                // Wir haben nun das Wort, jetzt müssen wir es codieren
                let mut _raw = String::new();
                let code = match _char {
                    'a'..='z' => (_char as u8 - b'a' + 1).to_string(),
                    'A'..='Z' => (_char as u8 - b'A' + 1).to_string(),
                    '0'..='9' => format!("00{}", _char),
                    _ => _char.to_string(),
                };
                _raw.push_str(&code);
                _liste.push(_raw);
            }
            _liste.push("0".to_string()); // Zeilentrenner hinzufügen
        }
        _result = _liste.join(" ");
        self.result = _result;
    }

}