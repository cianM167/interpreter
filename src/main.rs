use std::env;
use std::fs::read_to_string;

use crate::lexer::lexer;
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

fn parse(string: &str) -> String {
    //println!("in string:{}", string);
    let mut res: String = string.to_string().replace(" ", "");
    let mut out: String;

    let matches = tokenize(&res);

    for mat in matches {
        out = parse(&mat);
        let tok_pos = first_token(&res);
        res = res.replace(tok_pos, &out);
        //println!("new string{}", res);
    }

    if res.contains("<") {
        let parts = res.split_once("<").unwrap();

        let val1 = parse(parts.0);
        let val2 = parse(parts.1);

        res = (val1 > val2).to_string();
    }

    if res.contains(">") {
        let parts = res.split_once(">").unwrap();

        let val1 = parse(parts.0);
        let val2 = parse(parts.1);

        res = (val1 < val2).to_string();
    }

    if res.contains("==") {
        let parts = res.split_once("==").unwrap();

        let val1 = parse(parts.0);
        let val2 = parse(parts.1);

        res = (val1 == val2).to_string();
    }

    if res.contains("-") {
        let parts = res.split_once("-").unwrap();

        let val1: f32 = parse(parts.0).parse().unwrap();
        let val2: f32 = parse(parts.1).parse().unwrap();

        res = (val1 - val2).to_string();
    }

    if res.contains("+") {
        let parts = res.split_once("+").unwrap();

        let val1: f32 = parse(parts.0).parse().unwrap();
        let val2: f32 = parse(parts.1).parse().unwrap();

        res = (val1 + val2).to_string();
    }

    if res.contains("/") {
        let parts = res.split_once("/").unwrap();

        let val1: f32 = parse(parts.0).parse().unwrap();
        let val2: f32 = parse(parts.1).parse().unwrap();

        res = (val1 / val2).to_string();
    }

    if res.contains("*") {
        let parts = res.split_once("*").unwrap();

        let val1: f32 = parse(parts.0).parse().unwrap();
        let val2: f32 = parse(parts.1).parse().unwrap();

        res = (val1 * val2).to_string();
    }
    
    //println!("returning:{}", res);
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

fn print(p_string: &str, var_vec: &Vec<Variable>) {
    let arguments_string: String = tokenize(p_string)[0].to_string();
    let arguments = arguments_string.split(",");

    for argument in arguments {
        if argument.contains('"') {
            let to_print = &(argument.replace(&['"'], ""));
            print!("{}",to_print);
        } else {
            let to_print = parse(&replace_varname_in_string(&argument, &var_vec));
            
            print!("{}", to_print);
            
        }

    }

    print!("\n");
}

fn main() {
    let args: Vec<String> = env::args().collect(); 
    //println!("{:?}",args);
    //let variable_name = fs::read_to_string(args[1].clone()).unwrap();

    let file_vec = read_lines(&(args[1]));
    let tokens = lexer(file_vec.clone());//this is really really slow just here so it compiles
    println!("{:?}", tokens);
    let mut var_vec: Vec<Variable> = vec![];
    for (line_number, line) in file_vec.iter().enumerate() {
        if !line.is_empty() {
            let start = (line.split(" ")).next().unwrap();
            let (i, in_vec) = is_in_vec_tup(&var_vec, start.into());

            if line.starts_with("print") {
                let arg = line.split_once("print").unwrap();
                print(arg.1, &var_vec);

            } else if start == "let" {
                let arg = line.split_once(" ").unwrap();
                let assignment = &arg.1.replace(" ", "");
                let parts = assignment.split_once("=").unwrap();

                let variable_name = parts.0;
                let variable_value_string = parse(&replace_varname_in_string(parts.1, &var_vec));
                let variable_value: VariableType;

                if variable_value_string.contains(".") {
                    variable_value = VariableType::Float(variable_value_string.parse().unwrap());
                } else if variable_value_string == "true" || variable_value_string == "false" {
                    variable_value =
                        if variable_value_string == "true" {
                            VariableType::Bool(true)
                        } else {
                            VariableType::Bool(false)
                        };
                } else {
                    variable_value = VariableType::Integer(variable_value_string.parse().unwrap());
                }

                let new_variable: Variable = Variable { 
                    name: variable_name.to_string(),
                     val: variable_value,
                };

                var_vec.push(new_variable);

            } else if line.starts_with("if") {
                let arg = line.split_once("if").unwrap();
                let mut comp = arg.1;
                if comp.ends_with("{") {

                }
            
            } else if in_vec { 
                if line.contains("=") {
                    let arg = line.split_once("=").unwrap();
                    if !arg.1.contains("=") {
                        //println!("after equal:{}", arg.1);
                        //perform_operation(arg.1.into(), &mut var_vec, i);// variable is mutated by function
                        let raw_arg = arg.1.replace(" ", "");
                        let variable_value_string = parse(&replace_varname_in_string(arg.1, &var_vec));
                        let variable_value: VariableType;
                        if variable_value_string.contains(".") {
                            variable_value = VariableType::Float(variable_value_string.parse().unwrap());
                        } else if raw_arg == "true" || raw_arg == "false" {
                            variable_value =
                                if variable_value_string == "true" {
                                    VariableType::Bool(true)
                                } else {
                                    VariableType::Bool(false)
                                };
                        } else {
                            //println!("assigning int");
                            variable_value = VariableType::Integer(variable_value_string.parse().unwrap());      
                        }
                        var_vec[i].mutate(variable_value);

                    } else {
                        println!("incorrect assignment only one = allowed");
                        return;
                    }
                }
            } else if line.starts_with("//") {
    
            } else {
                println!("Error on line:{} Couldnt parse:{}.\nExiting", line_number, line,);
                return;
            }
        }
    }
}