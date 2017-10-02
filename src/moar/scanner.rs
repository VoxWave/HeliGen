pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    scanner_state: ScannerState,
    buffer_string: String,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner{
            source,
            tokens: Vec::new(),
            scanner_state: ScannerState::Neutral,
            buffer_string: String::new();
        }
    }

    pub fn scan(&mut self) -> Vec<Token> {
        for c in source.chars() {
            match scanner_state {
                Neutral => self.neutral_scan(&c),
            }
        }
        tokens.clone()
    }

    fn neutral_scan(&mut self, c: &char) {
        match *c {
            '{' => tokens.push
        }
    } 
}

enum ScannerState {
    Neutral,
}

enum Token {
    CurlyBracket(Direction),
    Bracket(Direction),
    Identifier(String),
    
}