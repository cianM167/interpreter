enum TokenType {
    //single character tokens
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus, Semicolon, Slash, Star,

    //One or two character tokens
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,

    //Literals
    Identifier, String(String), Number,

    //Keywords
    And, Struct, Else, False, Fun, For, If, Nil, Or,
    Print, Return, True, Let, While,

    Eof
}

enum ScanResult {
    Success,
    EndLine,
    Error(String),
}

enum StringResult {
    Unterminated,
    Found(String),
}

enum ScanTokensResult {
    Success,
    Error(String),
}

fn scan_token(it: &mut impl Iterator<Item = char>, vec: &mut Vec<TokenType>) -> ScanResult {
    let c: char;
    match it.next() {
        None => return ScanResult::EndLine,
        Some(next) => c = next,
    }
    match c {
        '(' => vec.push(TokenType::LeftParen),
        ')' => vec.push(TokenType::RightParen),
        '{' => vec.push(TokenType::LeftBrace),
        '}' => vec.push(TokenType::RightBrace),
        ',' => vec.push(TokenType::Comma),
        '.' => vec.push(TokenType::Dot),
        '-' => vec.push(TokenType::Minus),
        '+' => vec.push(TokenType::Plus),
        ';' => vec.push(TokenType::Semicolon),
        '*' => vec.push(TokenType::Star),
        '!' => {
            if matches(it, '=') {
                vec.push(TokenType::BangEqual);
            } else {
                vec.push(TokenType::Bang);
            };
        }
        '=' => {
            if matches(it, '=') {
                vec.push(TokenType::EqualEqual);
            } else {
                vec.push(TokenType::Equal);
            };
        }
        '<' => {
            if matches(it, '=') {
                vec.push(TokenType::LessEqual);
            } else {
                vec.push(TokenType::Less);
            };
        }
        '>' => {
            if matches(it, '=') {
                vec.push(TokenType::GreaterEqual);
            } else {
                vec.push(TokenType::Greater);
            };
        }
        '/' => {
            if matches(it, '/') {
                return ScanResult::EndLine;
            } else {
                vec.push(TokenType::Slash);
            }
        }
        ' ' => return ScanResult::Success,
        '\r' => return ScanResult::Success,
        '\t' => return ScanResult::Success,
        '\n' => return ScanResult::EndLine,
        '"' => {
            match string(it) {
                StringResult::Unterminated => return ScanResult::Error("Unterminated string".to_string()),
                StringResult::Found(string) => {
                    vec.push(TokenType::String(string));
                }
            }
        }

        er => return ScanResult::Error("Character not recognised".to_string())
    }

    return ScanResult::Success;
}

fn matches(it: &mut impl Iterator<Item = char>, expected: char) -> bool {
    match it.next() {
        None => return false,
        Some(next) => {
            if next == expected {
                return true;
            } else {
                return false;
            }
        }
    }
}

fn string(it: &mut impl Iterator<Item = char>) -> StringResult {
    let mut string = "".to_string();
    loop {
        match it.next() {
            None => return StringResult::Unterminated,
            Some(next) => {
                if next == '"' {
                    return StringResult::Found(string);
                } else {
                    string.push(next);
                }
            }
        }
    }
}

fn scan_tokens(str: String) -> ScanTokensResult {
    let mut tokens:Vec<TokenType> = vec![];
    let mut iter: std::str::Chars<'_> = str.chars().into_iter();
    let mut end  = false;
    while !end {
        match scan_token(&mut iter, &mut tokens) {
            ScanResult::EndLine => end = true,
            ScanResult::Error(message) => return ScanTokensResult::Error(message),//add line number to error
            ScanResult::Success => end = false
        }
    }

    return ScanTokensResult::Success;
    
}

fn lexer() {

}



fn main() {
    scan_tokens("(),.".to_string());
}
