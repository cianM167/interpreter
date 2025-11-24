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

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

fn main() {
    let args: Vec<String> = env::args().collect();
    //println!("{:?}",args);
    //let variable_name = fs::read_to_string(args[1].clone()).unwrap();

    let file_vec = read_lines(&(args[1]));
    let mut var_vec: Vec<Variable<u16>>;
    let mut line_number = 0;
    for line in file_vec {
        line_number += 1;
        if line.starts_with("print") {
            let mut arg = line.split("(");
            //println!("{}", arg.next().unwrap());
            arg.next();
            let mut pString = arg.next().unwrap(); 
            let pString = &(pString.replace(&['(',')','"'], ""));
            println!("{}",pString);
           
        } else if line.starts_with("let") {
            let mut arg = line.split(" ");
            arg.next();
            let variable_name = arg.next().unwrap();
            let mut arg = line.split("=");
            arg.next();
            let variable_value = Some(((arg.next().unwrap()).replace(" ", "")).parse().unwrap());
            //unsafe {
                //let layout = Layout::new::<u16>();
                //let ptr = alloc(layout);
                //*(ptr as *mut u16) = variable_value;
                //println!("value:{}",*ptr);
            //}
            let &mut new_var: &mut Variable<u16> = &mut Variable {
                name: variable_name.to_string(),
                val: variable_value,
            };
            var_vec.push(new_var);
            println!("{}", var_vec[0].val.unwrap());

            

        } else if line.is_empty() {
            
        } else if line.starts_with("//") {

        } else {
            println!("Error on line:{} Couldnt parse:{}.\nExiting", line_number, line,);
            return;
        }
    }
}
