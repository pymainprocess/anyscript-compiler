use std::ffi::CString;
use std::path::{Path, PathBuf};
use std::fs::{File, write};
use std::io::{BufReader, BufWriter, Read};

pub struct LynchReader {
    filepath: String,
    content: String,
    coded: String,
    encoded: String,
}

impl LynchReader {

    pub fn new(filepath: String) -> Self {
        LynchReader {
            filepath,
            content: String::new(),
            coded: String::new(),
            encoded: String::new(),
        }
    }

    pub fn from_cstring(filepath: CString) -> Self {
        let _string = filepath.to_str().unwrap().to_string();
        Self {
            filepath: _string.clone(),
            content: String::new(),
            coded: String::new(),
            encoded: String::new(),
        }
    }

    pub fn content_to_string(&mut self) -> String {
        let _file = File::open(&self.filepath).expect("Could not open file.");
        let mut _content = String::new();
        let mut reader = BufReader::new(_file);
        reader.read_to_string(&mut _content).expect("Could not read file.");
        _content
    }

    pub fn content_to_cstring(&mut self) -> CString {
        let _content = self.content_to_string();
        CString::new(_content).expect("Could not convert string to cstring.")
    }

    pub fn read_content(&mut self) {
        let _content = self.content_to_string();
        self.content = _content;
    }

    pub fn decode_lynch(&mut self) {
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
        self.coded = _result;
    }

    pub fn encode_lynch(&mut self) {
        let mut _result = String::new();
        let mut _encoded_list = Vec::new();
        let coded_content = &self.coded;

        for code in coded_content.split_whitespace() {
            let encoded_char = match code {
                "1" => 'a',
                "2" => 'b',
                "3" => 'c',
                "4" => 'd',
                "5" => 'e',
                "6" => 'f',
                "7" => 'g',
                "8" => 'h',
                "9" => 'i',
                "10" => 'j',
                "11" => 'k',
                "12" => 'l',
                "13" => 'm',
                "14" => 'n',
                "15" => 'o',
                "16" => 'p',
                "17" => 'q',
                "18" => 'r',
                "19" => 's',
                "20" => 't',
                "21" => 'u',
                "22" => 'v',
                "23" => 'w',
                "24" => 'x',
                "25" => 'y',
                "26" => 'z',
                "00" => '0',
                "001" => '1',
                "002" => '2',
                "003" => '3',
                "004" => '4',
                "005" => '5',
                "006" => '6',
                "007" => '7',
                "008" => '8',
                "009" => '9',
                "0" => '\n', // Zeilentrenner
                _ => '?', // Unbekanntes Zeichen
            };
            _encoded_list.push(encoded_char);
        }

        _result = _encoded_list.iter().collect::<String>();
        self.encoded = _result;
    }

    pub fn coded_to_string(&mut self) -> String {
        self.decode_lynch();
        let _result = self.encoded.clone();
        _result
    }

    pub fn coded_to_cstring(&mut self) -> CString {
        let _result = self.coded_to_string();
        CString::new(_result).expect("Could not convert string to cstring.")
    }

    pub fn encoded_to_string(&mut self) -> String {
        self.encode_lynch();
        let _result = self.encoded.clone();
        _result
    }

    pub fn encoded_to_cstring(&mut self) -> CString {
        let _result = self.encoded_to_string();
        CString::new(_result).expect("Could not convert string to cstring.")
    }
}