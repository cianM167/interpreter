use std::fs;
use std::env;
use std::fs::read_to_string;
use std::mem::replace;
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

fn is_in_vec(vec: &Vec<Variable<u16>>, varname: String) -> bool {
    for Variable in vec {
        if Variable.name == varname {
            return true;
        }
    }
    return false;
}

fn print(p_string: &str) {
    if p_string.contains('"') {
        let p_string = &(p_string.replace(&['(',')','"'], ""));
        println!("{}",p_string);
    } else {
        println!("{}",p_string);
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
                print(arg.1);

            } else if start == "let" {
                let mut arg = line.split(" ");
                arg.next();
                let variable_name = arg.next().unwrap();
                let mut arg = line.split("=");
                arg.next();
                let variable_value = Some(((arg.next().unwrap()).replace(" ", "")).parse().unwrap());

                let new_var: Variable<u32> = Variable {
                    name: variable_name.to_string(),
                    val: variable_value,
                };
                let newenum = VariableTypes::Integer(new_var);
                var_vec.push(newenum);
                if let VariableTypes::Integer(new_var) = newenum {
                    println!("value: {}", new_var.val.unwrap());
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
