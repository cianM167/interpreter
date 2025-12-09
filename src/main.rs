use std::{char::ToLowercase, env};
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
    name: String,
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

fn parse(tokens: &[TokenType]) -> String {
    let mut res: String = "".to_string();
    for (i,token) in tokens.iter().enumerate() {
        match token {//handle operators
            TokenType::Less => {
                let value1 = parse(&tokens[0..i]);
                let value2 = parse(&tokens[i..]);
            },
            TokenType::Greater => {

            },
            TokenType::GreaterEqual => {

            },
            TokenType::LessEqual => {

            },
            TokenType::BangEqual => {

            },
            TokenType::Minus => {

            }
            TokenType::Plus => {
                //parse stuff before plus
                println!("{:?}", &tokens[0..i]);
                let value1: f32 = parse(&tokens[0..i]).parse().unwrap();
                //parse stuff after it
                let value2: f32 = parse(&tokens[i..]).parse().unwrap();

                res = (value1 + value2).to_string();
            },
            TokenType::Slash => {

            },
            TokenType::Star => {

            }
            _ => {

            }
        }
    }

    return res;
}

fn tokenize(value: &str) -> Vec<String> { //returns tokens inside "()"
    let mut depth = 0;
    let mut tok_start = 0;
    let mut tok_end;
    let mut token_vector: Vec<String> = vec![];
    for (i,c) in value.chars().enumerate() {
        if c == '(' {
            if depth == 0 {
                tok_start = i + 1;
            }
            depth += 1;
        } else if c == ')' {
            depth -= 1;
            if depth == 0 {
                tok_end = i;

                token_vector.push(value[tok_start..tok_end].to_string());
                println!("token vector:{:?}", token_vector);
            } 

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

fn is_in_vec(vec: &Vec<Variable>, varname: String) -> bool {
    for variable in vec {
        if variable.name == varname {
            return true;
        }
    }
    return false;
}

fn is_in_vec_tup(vec: &Vec<Variable>, varname: String) -> (usize, bool) {
    for (i, variable) in vec.iter().enumerate() {
        if variable.name == varname {
            return (i, true);
        }
    }
    return (0, false);
}

fn replace_varname_in_string(value: &str, var_vec: &Vec<Variable>) -> String {
    let mut string = value.to_string();
    for variable in  var_vec {

        match &variable.val {
            VariableType::Integer(variable_type) => {
                if value.contains(&variable.name) {
                    string = string.replace(&variable.name, &variable_type.to_string());
                }
            }
            VariableType::Float(variable_type) => {
                if value.contains(&variable.name) {
                    string = string.replace(&variable.name, &variable_type.to_string());
                }
            }
            VariableType::Bool(variable_type) => {
                if value.contains(&variable.name) {
                    string = string.replace(&variable.name, &variable_type.to_string());
                }
            }
            VariableType::String(variable_type) => {
                if value.contains(&variable.name) {
                    string = string.replace(&variable.name, &variable_type.to_string());
                }
            }
        }
    }

    return string;
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
    let tokens = lexer(file_vec.clone());//this is really really slow just here so it compiles
    println!("{:?}", tokens);
    println!("{}",parse(&tokens[..]));

    let mut token_iter = tokens.into_iter().peekable();
    while let Some(token) = token_iter.next() {
        println!("{:?}", token);
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