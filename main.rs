use std::fs;
use std::env;
use std::fs::read_to_string;
use std::mem::replace;
use regex::Regex;
use std::alloc::{alloc, dealloc, Layout};

//create variable struct
struct variable {
    name: String,
    ptr: *mut u8,
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

    let fileVec = read_lines(&(args[1]));
    let mut line_number = 0;
    for line in fileVec {
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
            let variable_value = ((arg.next().unwrap()).replace(" ", "")).parse().unwrap();
            unsafe {
                let layout = Layout::new::<u16>();
                let ptr = alloc(layout);
                *(ptr as *mut u16) = variable_value;
                println!("value:{}",*ptr);
            }
            

            

        } else if line.is_empty() {
            
        } else if line.starts_with("//") {

        } else {
            println!("Error on line:{} Couldnt parse:{}.\nExiting", line_number, line,);
            return;
        }
    }
}
