use std::fmt;
use phf::phf_map;

#[derive(Debug, Clone)]
pub enum TokenType {
    //single character tokens
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comma, Minus, Plus, Colon, Semicolon, Slash, Star,

    //One or two character tokens
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,
    Dot, DotDot,

    //Literals
    Identifier(String), String(String), Integer(i32), Float(f32),

    //Keywords
    And, Struct, Else, False, Fun, For, If, Nil, Or,
    Print, Return, True, Let, While,

    //End of file
    Eof,
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

static KEYWORDS: phf::Map<&'static str, TokenType> = phf_map! {//constant hashmap for reserved words
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
    "struct" => TokenType::Struct,
};

fn scan_token(it: &mut impl Iterator<Item = char>, token: &mut Vec<TokenType>) -> ScanResult {
    let c: char;
    match it.next() {
        None => return ScanResult::EndLine,
        Some(next) => c = next,
    }
    match c {
        '(' => token.push(TokenType::LeftParen),
        ')' => token.push(TokenType::RightParen),
        '{' => token.push(TokenType::LeftBrace),
        '}' => token.push(TokenType::RightBrace),
        ',' => token.push(TokenType::Comma),
        '-' => token.push(TokenType::Minus),
        '+' => token.push(TokenType::Plus),
        ';' => token.push(TokenType::Semicolon),
        '*' => token.push(TokenType::Star),
        ':' => token.push(TokenType::Colon),
        '!' => {
            if matches(it, '=') {
                token.push(TokenType::BangEqual);
            } else {
                token.push(TokenType::Bang);
            };
        }
        '=' => {
            if matches(it, '=') {
                token.push(TokenType::EqualEqual);
            } else {
                token.push(TokenType::Equal);
            };
        }
        '<' => {
            if matches(it, '=') {
                token.push(TokenType::LessEqual);
            } else {
                token.push(TokenType::Less);
            };
        }
        '>' => {
            if matches(it, '=') {
                token.push(TokenType::GreaterEqual);
            } else {
                token.push(TokenType::Greater);
            };
        }
        '/' => {
            if matches(it, '/') {
                return ScanResult::EndLine;
            } else {
                token.push(TokenType::Slash);
            }
        }
        '.' => {
            if matches(it,'.') {
                token.push(TokenType::DotDot);
            } else {
                token.push(TokenType::Dot);
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
                    token.push(TokenType::String(string));
                }
            };
        }

        default => {
            //println!("{}", default);
            if default.is_digit(10) {
                let number = number(c, it);
                if number.contains(".") {
                    token.push(TokenType::Float(number.parse().unwrap()));//add case for unwrap failing
                } else {
                    token.push(TokenType::Integer(number.parse().unwrap()));
                }

            } else if default.is_alphanumeric() {
                let identifier = identifier(default, it);
                match KEYWORDS.get(&identifier) {
                    Some(keyword) => {
                        token.push(keyword.clone());//there might be a better way to do this
                    }
                    _ => token.push(TokenType::Identifier(identifier)),
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
        //println!("Number:{}", number);
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

fn scan_tokens(tokens: &mut Vec<TokenType>, file_vec: Vec<String>) {
    for (line_number, line) in file_vec.iter().enumerate() {
        let mut iter = line.chars().into_iter().peekable();
        let mut end  = false;
        while !end {
            match scan_token(&mut iter, tokens) {
                ScanResult::EndLine => {
                    end = true;
                },
                ScanResult::Error(message) => { 
                    panic!("{} on line:{}", message, line_number)//add line number to error
                },
                ScanResult::Success => end = false,
            }
        }
    }
  
}

fn identifier(identifier_start: char, it: &mut impl Iterator<Item = char>) -> String {
    let mut peek_iter = it.peekable();
    let mut identifier: String = "".into();
    identifier += &identifier_start.to_string();

    loop {
        //println!("Identifier:{}", identifier);
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

pub fn lexer(file_vec: Vec<String>) -> Vec<TokenType> {
    let mut tokens:Vec<TokenType> = vec![];
    scan_tokens(&mut tokens, file_vec);
    tokens.push(TokenType::Eof);
    return tokens
}