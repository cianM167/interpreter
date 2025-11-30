use std::{env, string};
use std::fs::read_to_string;

//create variable struct
struct Variable<T> {
    name: String,
    val: Option<T>,
}

enum VariableTypes {
    Integer(Variable<u32>),
    Float(Variable<f32>),
    String(Variable<String>)
}

impl VariableTypes {
    pub fn mutate(&mut self, value: String) {
        match self {
            VariableTypes::Integer(c) => {
                (*c).val = Some(value.parse().unwrap());
            },
            VariableTypes::Float(c) => {
                (*c).val = Some(value.parse().unwrap());
            }
            VariableTypes::String(c) => {
                (*c).val = Some(value.parse().unwrap());
            }
        }
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
    println!("in string:{}", string);
    let mut res: String = string.to_string().replace(" ", "");
    let mut out: String;

    let matches = tokenize(&res);

    for mat in matches {
        out = parse(&mat);
        let tok_pos = first_token(&res);
        res = res.replace(tok_pos, &out);
        println!("new string{}", res);
    }

    if res.contains("-") {
        let parts = res.split_once("-").unwrap();

        let val1: u32 = parse(parts.0).parse().unwrap();
        let val2: u32 = parse(parts.1).parse().unwrap();

        res = (val1 - val2).to_string();
    }

    if res.contains("+") {
        let parts = res.split_once("+").unwrap();

        let val1: u32 = parse(parts.0).parse().unwrap();
        let val2: u32 = parse(parts.1).parse().unwrap();

        res = (val1 + val2).to_string();
    }

    if res.contains("*") {
        let parts = res.split_once("*").unwrap();

        let val1: u32 = parts.0.parse().unwrap();
        let val2: u32 = parts.1.parse().unwrap();

        res = (val1 * val2).to_string();
    }
    
    println!("returning:{}", res);
    return res;
}

fn tokenize(value: &str) -> Vec<String> { //returns tokens inside "()"
    let mut depth = 0;
    let mut tok_start = 0;
    let mut tok_end = 0;
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
                println!("{:?}", token_vector);
            } 

        }
    }

    return token_vector;
}

fn first_token(value: &str) -> &str { //returns substring
    let mut depth = 0;
    let mut tok_start = 0;
    let mut tok_end = 0;

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

fn perform_operation(oper_string: String, var_vec: &mut Vec<VariableTypes>, var_index: usize) {
    let ref dir: &mut VariableTypes  = &mut var_vec[var_index];
    let mut outstring: String = "".into();
    match dir {
        VariableTypes::Integer(_variable) => {
                if oper_string.contains("+") {
                    let cleanedstring = oper_string.replace(" ", "");
                    let mut chunks = cleanedstring.split("+");
                    //println!("{:?}", chunks.next().unwrap());
                    let val1: u32 = chunks.next().unwrap().parse().unwrap();//failing
                    let val2: u32 = chunks.next().unwrap().parse().unwrap();
                    
                    outstring = (val1 + val2).to_string();
                    
                }
            }
            VariableTypes::Float(_variable) => {
                if oper_string.contains("+") {
                    let cleanedstring = oper_string.replace(" ", "");
                    let mut chunks = cleanedstring.split("+");
                    //println!("{:?}", chunks.next().unwrap());
                    let val1: f32 = chunks.next().unwrap().parse().unwrap();//failing
                    let val2: f32 = chunks.next().unwrap().parse().unwrap();
                    
                    outstring = (val1 + val2).to_string();
                    
                }
            }
            VariableTypes::String(_variable) => {
                if oper_string.contains("+") {
                    let cleanedstring = oper_string.replace(" ", "");
                    let mut chunks = cleanedstring.split("+");
                    //println!("{:?}", chunks.next().unwrap());
                    let val1: String = chunks.next().unwrap().into();
                    let val2: String = chunks.next().unwrap().to_owned();
                    
                    outstring = (val1 + &val2).to_string();
                    
                }
            }
    }

    var_vec[var_index].mutate(outstring);
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
            VariableTypes::String(variable) => {
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
            VariableTypes::String(variable) => {
                if variable.name == varname {
                    return (i, true);
                }
            }
        }
    }
    return (0, false);
}

fn replace_varname_in_string(value: &str, var_vec: &Vec<VariableTypes>) -> String {
    let mut string = value.to_string();
    for variable in  var_vec {
        match variable {
            VariableTypes::Integer(variable) => {
                if value.contains(&variable.name) {
                    string = string.replace(&variable.name, &variable.val.unwrap().to_string());
                }
            }
            VariableTypes::Float(variable) => {

            }
            VariableTypes::String(Variable) => {

            }
        }
    }

    return value.to_string();
}

fn print(p_string: &str, var_vec: &Vec<VariableTypes>) {
    if p_string.contains('"') {
        let p_string = &(p_string.replace(&['(',')','"'], ""));
        print!("{}",p_string);
    } else {
        let mut to_print = parse(&replace_varname_in_string(p_string, &var_vec));
        
        println!("{}", to_print);
        

        //let p_string = &(p_string.replace(&[')'],""));
        //let (i, ans) = is_in_vec_tup(var_vec, p_string.into());
        //if ans {
            //print!("{}",display_enum(&var_vec[i]));
        //} else {
            //println!("Variable:{} is not declared", p_string);
        //}
    }
}

fn display_enum(var: &VariableTypes) -> String {
    match var {
        VariableTypes::Integer(variable) => variable.val.unwrap().to_string(),
        VariableTypes::Float(variable) => variable.val.unwrap().to_string(),
        VariableTypes::String(variable) => variable.val.as_ref().unwrap().to_string(),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    //println!("{:?}",args);
    //let variable_name = fs::read_to_string(args[1].clone()).unwrap();

    let file_vec = read_lines(&(args[1]));
    let mut var_vec: Vec<VariableTypes> = vec![];
    for (line_number, line) in file_vec.iter().enumerate() {
        if !line.is_empty() {
            let start = (line.split(" ")).next().unwrap();
            let (i, in_vec) = is_in_vec_tup(&var_vec, start.into());

            if line.starts_with("print") {
                let arg = line.split_once("print").unwrap();
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

            } else if in_vec { 
                if line.contains("=") {
                    let arg = line.split_once("=").unwrap();
                    if !arg.1.contains("=") {
                        //println!("after equal:{}", arg.1);
                        perform_operation(arg.1.into(), &mut var_vec, i);// variable is mutated by function
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