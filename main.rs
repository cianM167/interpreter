use std::fs;
use std::env;
use std::fs::read_to_string;
use std::mem::replace;
use std::result;
use regex::Regex;
use std::alloc::{alloc, dealloc, Layout};

//create variable struct
struct Variable<T> {
    name: String,
    val: Option<T>,
}

enum VariableTypes {
    Integer(Variable<u32>),
    Float(Variable<f32>),
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

fn perform_operation(operString: String) -> u16{
    if operString.contains("+") {
        let cleanedString = operString.replace(" ", "");
        let mut chunks = cleanedString.split("+");
        let val1: u16 = chunks.next().unwrap().parse().unwrap();
        let val2: u16 = chunks.next().unwrap().parse().unwrap();

        println!("{}", val1 + val2);
        return val1 + val2;
    } else {
        println!("Invalid operator");
        return 0;
    }
}

fn is_in_vec(vec: &Vec<VariableTypes>, varname: String) -> bool {
    for variable in vec {
        match variable {
            VariableTypes::Integer(variable) => {
                if variable.name == varname {
                    return true;
                }
            }
            VariableTypes::Float(variable) => {
                if variable.name == varname {
                    return true;
                }
            }
        }
    }
    return false;
}

fn is_in_vec_tup(vec: &Vec<VariableTypes>, varname: String) -> (usize, bool) {
    for (i, variable) in vec.iter().enumerate() {
        match variable {
            VariableTypes::Integer(variable) => {
                if variable.name == varname {
                    return (i, true);
                }
            }
            VariableTypes::Float(variable) => {
                if variable.name == varname {
                    return (i, true);
                }
            }
        }
    }
    return (0, false);
}

fn print(p_string: &str, var_vec: &Vec<VariableTypes>) {
    if p_string.contains('"') {
        let p_string = &(p_string.replace(&['(',')','"'], ""));
        println!("{}",p_string);
    } else {
        let p_string = &(p_string.replace(&[')'],""));
        let (i, ans) = is_in_vec_tup(var_vec, p_string.into());
        if ans {
            println!("{}",display_enum(&var_vec[i]));
        } else {
            println!("Variable:{} is not declared", p_string);
        }
    }
}

fn display_enum(var: &VariableTypes) -> String {
    match var {
        VariableTypes::Integer(variable) => variable.val.unwrap().to_string(),
        VariableTypes::Float(variable) => variable.val.unwrap().to_string(),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    //println!("{:?}",args);
    //let variable_name = fs::read_to_string(args[1].clone()).unwrap();

    let file_vec = read_lines(&(args[1]));
    let mut var_vec: Vec<VariableTypes> = vec![];
    let mut line_number = 0;
    for line in file_vec {
        line_number += 1;
        if !line.is_empty() {
            let start = (line.split(" ")).next().unwrap();

            if line.starts_with("print") {
                let mut arg = line.split_once("(").unwrap();
                print(arg.1, &var_vec);

            } else if start == "let" {
                let mut arg = line.split(" ");
                arg.next();
                let variable_name = arg.next().unwrap();
                let mut arg = line.split("=");
                arg.next();
                let value  = arg.next().unwrap().replace(" ", "");
                if value.contains(".") { //checking if var is a float
                    let variable_value = Some((value).parse().unwrap());

                    let new_var: Variable<f32> = Variable {
                        name: variable_name.to_string(),
                        val: variable_value,
                    };
                    let newenum = VariableTypes::Float(new_var);
                    var_vec.push(newenum);
                    //println!("{}",display_enum(&var_vec[1]));
                } else { //otherwise its an int
                    let variable_value = Some((value).parse().unwrap());

                    let new_var: Variable<u32> = Variable {
                        name: variable_name.to_string(),
                        val: variable_value,
                    };
                    let newenum = VariableTypes::Integer(new_var);
                    var_vec.push(newenum);
                    //println!("{}",display_enum(&var_vec[0]));
                    //println!("Testing find func{}", is_in_vec(&var_vec, "a".into()))
                }

            //} else if is_in_vec(&var_vec, start.to_string()) {  
                
            } else if line.starts_with("//") {
    
            } else {
                println!("Error on line:{} Couldnt parse:{}.\nExiting", line_number, line,);
                return;
            }
        }
    }
}
