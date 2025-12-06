use std::fmt;
use phf::phf_map;

#[derive(Debug, Clone)]
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
    Identifier(String), String(String), Integer(i32), Float(f32),

    //Keywords
    And, Struct, Else, False, Fun, For, If, Nil, Or,
    Print, Return, True, Let, While,

    Eof
}

impl TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       write!(f, "{:?}", self)
    }
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

static Keywords: phf::Map<&'static str, TokenType> = phf_map! {//constant hashmap for reserved words
    "and" => TokenType::And,
    "else" => TokenType::Else,
    "false" => TokenType::False,
    "for" => TokenType::For,
    "fun" => TokenType::Fun,
    "if" => TokenType::If,
    "nil" => TokenType::Nil,
    "or" => TokenType::Or,
    "print" => TokenType::Print,
    "return" => TokenType::Return,
    "true" => TokenType::True,
    "let" => TokenType::Let,
    "while" => TokenType::While,
};

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
            };
        }

        default => {
            println!("{}", default);
            if (default.is_digit(10)) {
                let number = number(c, it);
                if number.contains(".") {
                    vec.push(TokenType::Float(number.parse().unwrap()));//add case for unwrap failing
                } else {
                    vec.push(TokenType::Integer(number.parse().unwrap()));
                }

            } else if default.is_alphanumeric() {
                let identifier = Identifier(default, it);
                match Keywords.get(&identifier) {
                    Some(keyword) => {
                        vec.push(keyword.clone());//there might be a better way to do this
                    }
                    _ => vec.push(TokenType::Identifier(identifier)),
                }
            } else {
               return ScanResult::Error("Character not recognised".to_string()) 
            };
        }
    }

    return ScanResult::Success;
}

fn matches(it: &mut impl Iterator<Item = char>, expected: char) -> bool {
    let mut peek_iter = it.peekable();

    match peek_iter.peek() {
        None => return false,
        Some(next) => {
            if *next == expected {
                peek_iter.next();
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

fn number(number_start: char, it: &mut impl Iterator<Item = char>) -> String {
    let mut peek_iter = it.peekable();
    let mut number: String = "".into();
    number += &number_start.to_string();

    loop {
        println!("Number:{}", number);
        match peek_iter.peek() {
            None => return number,
            Some(peeked) => {
                if peeked.is_digit(10) || *peeked == '.' {
                    let next = peek_iter.next().unwrap();
                    number += &next.to_string();
                } else {
                    return number;
                }
            }
        }
    }
}

fn scan_tokens(str: String) -> ScanTokensResult {
    let mut tokens:Vec<TokenType> = vec![];
    let mut iter = str.chars().into_iter().peekable();
    let mut end  = false;
    while !end {
        match scan_token(&mut iter, &mut tokens) {
            ScanResult::EndLine => {
                end = true;
            },
            ScanResult::Error(message) => { 
                return ScanTokensResult::Error(message)//add line number to error
            },
            ScanResult::Success => end = false,
        }
    }

    println!("{:?}", tokens);

    return ScanTokensResult::Success;
    
}

fn Identifier(identifier_start: char, it: &mut impl Iterator<Item = char>) -> String {
    let mut peek_iter = it.peekable();
    let mut identifier: String = "".into();
    identifier += &identifier_start.to_string();

    loop {
        println!("Identifier:{}", identifier);
        match peek_iter.peek() {
            None => return identifier,
            Some(peeked) => {
                if peeked.is_alphanumeric() || *peeked == '.' {
                    let next = peek_iter.next().unwrap();
                    identifier += &next.to_string();
                } else {
                    return identifier;
                }
            }
        }
    }
}

fn lexer() {

}



fn main() {
    scan_tokens("let a".into());
}