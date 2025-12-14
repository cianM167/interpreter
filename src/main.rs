use std::collections::HashMap;
use std::time::Duration;
use std::{env, thread::sleep};
use std::fs::read_to_string;

use crate::lexer::{TokenType, lexer};
mod lexer;

//create variable struct
enum VariableType {
    Integer(i32),
    Float(f32),
    String(String),
    Bool(bool),
}

struct Variable {
    val: VariableType,
}

impl Variable {
    fn mutate(&mut self, value: VariableType) {
        self.val = value;
    }
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

fn parse(tokens: &[TokenType]) -> TokenType {
    println!("Incoming slice{:?}", tokens);
    if tokens.len() == 1 {
        return tokens[0].clone();
    }
    //take slice in brackets

    for (i, token) in tokens.iter().enumerate() {
        match token {
            TokenType::LeftBrace => {
                return parse(remove_paren_from_slice(tokens));
            },
            TokenType::Minus => {
                let val1: TokenType = parse(&tokens[..i]);
                let val2: TokenType = parse(&tokens[i+1..]);
                return val1 - val2;
            },
            TokenType::Plus => {
                let val1: TokenType = parse(&tokens[..i]);
                let val2: TokenType = parse(&tokens[i+1..]);
                return val1 + val2;
            },
            TokenType::Slash => {
                let val1: TokenType = parse(&tokens[..i]);
                let val2: TokenType = parse(&tokens[i+1..]);
                return val1 / val2;
            },
            TokenType::Star => {
                let val1: TokenType = parse(&tokens[..i]);
                let val2: TokenType = parse(&tokens[i+1..]);
                return val1 * val2;
            },
            _ => (),
        }
        println!("{:?}", token);
    }
    return TokenType::Nil;
}

fn tokenize(tokens: &[TokenType]) -> Vec<&[TokenType]> { //returns tokens inside "()"
    let mut depth = 0;
    let mut start = 0;
    let mut end;
    let mut token_vector: Vec<&[TokenType]> = vec![];
    for (i,token) in tokens.iter().enumerate() {
        match token {
            TokenType::LeftParen => {
                if depth == 0 {
                    start = i + 1;
                }
                depth += 1;
            }
            TokenType::RightParen => {
                depth -= 1;
                if depth == 0 {
                    end = i;

                    token_vector.push(&tokens[start..end]);
                    println!("token vector:{:?}", token_vector);
                } 
            }
            _ => ()
        }
    }

    return token_vector;
}

fn remove_paren(token_iter: &mut impl Iterator<Item = TokenType>) -> Vec<TokenType> { //returns tokens inside "()"
    let mut depth = 0;
    let mut token_vector: Vec<TokenType> = vec![];
    while let Some(token) = token_iter.next()  {
        match token {
            TokenType::LeftParen => {
                if depth == 0 {
                    depth += 1;
                } else {
                    token_vector.push(token);
                }
            },
            TokenType::RightParen => {
                depth -= 1;
                if depth == 0 {
                    return token_vector;
                } else {
                    token_vector.push(token);
                }
            }
            default => {
                token_vector.push(default);
            }
        }
    }

    return token_vector;
}

fn remove_paren_from_slice(tokens: &[TokenType]) -> &[TokenType] { //returns tokens inside "()"
    let mut depth = 0;
    let mut start = 0;
    let mut i = 0;
    
    let mut token_iter = tokens.iter();
    while let Some(token) = token_iter.next()  {
        match token {
            TokenType::LeftParen => {
                if depth == 0 {
                    start = i + 1;
                    depth += 1;
                } else {
                    depth += 1;
                }
            },
            TokenType::RightParen => {
                depth -= 1;
                if depth == 0 {

                    return &tokens[start..i];
                }
            }
            _ => ()          
        }
        i += 1;
    }

    return &tokens[..];
}

fn first_token(value: &str) -> &str { //returns substring
    let mut depth = 0;
    let mut tok_start = 0;
    let tok_end;

    for (i,c) in value.chars().enumerate() {
        if c == '(' {
            if depth == 0 {
                tok_start = i;
            }
            depth += 1;
        } else if c == ')' {
            depth -= 1;
            if depth == 0 {
                tok_end = i + 1;

                return &value[tok_start..tok_end];
            } 

        }
    }

    return ""; //error out
}

fn print(tokens: Vec<TokenType>) {
    for token in tokens {
        match token {
            TokenType::String(value) => {
                print!("{value}");
            },
            TokenType::Comma => (),//do nothing
            TokenType::Integer(value) => {
                print!("{value}")
            },
            TokenType::Float(value) => {
                print!("{value}")
            },
            _ => {
                
            }
            
        }
    }    

}

fn main() {
    let args: Vec<String> = env::args().collect(); 
    //println!("{:?}",args);
    //let variable_name = fs::read_to_string(args[1].clone()).unwrap();

    let file_vec = read_lines(&(args[1]));
    let tokens = lexer(file_vec);//this is really really slow just here so it compiles
    println!("{:?}", tokens);
    println!("{:?}", parse(&tokens));
    //println!("{:?}", parse(&tokens[..]));
    let mut variables: HashMap<&str, Variable> = HashMap::new();

    let mut token_iter = tokens.into_iter().peekable();
    while let Some(token) = token_iter.next() {
        //println!("{:?}", token);
        match token {
            TokenType::Print => {
                //move forward capturing everything in print
                let to_print = remove_paren(&mut token_iter);
                print(to_print);
            },
            TokenType::Let => {

            },
            TokenType::If => {

            },
            TokenType::Identifier(value) => {

            },
            _ => {

            }

        }
    }
    
}