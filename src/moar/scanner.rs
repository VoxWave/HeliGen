pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    scanner_state: ScannerState,
    buffer_string: String,
}

impl Scanner {
    pub fn new() -> Self {
        
    }
}

enum ScannerState {
    Neutral,
}

enum Token {

}