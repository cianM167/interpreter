use std::env;
use std::fs::read_to_string;

//create variable struct
struct Variable<T> {
    name: String,
    val: Option<T>,
}

enum VariableTypes {
    Integer(Variable<u32>),
    Float(Variable<f32>),
    String(Variable<String>),
    Bool(Variable<bool>),
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
            VariableTypes::Bool(c) => {
                (*c).val = if value == "true" {
                            Some(true)
                        } else {
                            Some(false)
                        };
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

    if res.contains("==") {
        
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

        let val1: f32 = parts.0.parse().unwrap();
        let val2: f32 = parts.1.parse().unwrap();

        res = (val1 / val2).to_string();
    }

    if res.contains("*") {
        let parts = res.split_once("*").unwrap();

        let val1: f32 = parts.0.parse().unwrap();
        let val2: f32 = parts.1.parse().unwrap();

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
                //println!("{:?}", token_vector);
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
            VariableTypes::Bool(variable) => {
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
            VariableTypes::Bool(variable) => {
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
                if value.contains(&variable.name) {
                    string = string.replace(&variable.name, &variable.val.unwrap().to_string());
                }
            }
            VariableTypes::String(Variable) => {
                string = string.replace(&Variable.name, &Variable.val.clone().unwrap().to_string());
            }
            VariableTypes::Bool(Variable) => {
                string = string.replace(&Variable.name, &Variable.val.unwrap().to_string());
            }
        }
    }

    return string;
}

fn print(p_string: &str, var_vec: &Vec<VariableTypes>) {
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

fn display_enum(var: &VariableTypes) -> String {
    match var {
        VariableTypes::Integer(variable) => variable.val.unwrap().to_string(),
        VariableTypes::Float(variable) => variable.val.unwrap().to_string(),
        VariableTypes::String(variable) => variable.val.as_ref().unwrap().to_string(),
        VariableTypes::Bool(variable) => variable.val.as_ref().unwrap().to_string(),
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
                let arg = line.split_once(" ").unwrap();
                let assignment = &arg.1.replace(" ", "");
                let parts = assignment.split_once("=").unwrap();

                let variable_name = parts.0;
                let variable_value_string = parse(&replace_varname_in_string(parts.1, &var_vec));

                let new_enum: VariableTypes;
                if variable_value_string.contains(".") {
                    let variable_value: Option<f32> = Some(variable_value_string.parse().unwrap());

                    let new_variable: Variable<f32> = Variable {
                        name: variable_name.to_string(),
                        val: variable_value,
                    };
                    new_enum = VariableTypes::Float(new_variable);

                } else if variable_value_string == "true" || variable_value_string == "false" {

                    let variable_value =
                        if variable_value_string == "true" {
                            true
                        } else {
                            false
                        };

                    let new_variable: Variable<bool> = Variable {
                        name: variable_name.to_string(),
                        val: Some(variable_value),
                    };
                    new_enum = VariableTypes::Bool(new_variable);

                } else {
                    let variable_value: Option<u32> = Some(variable_value_string.parse().unwrap());

                    let new_variable: Variable<u32> = Variable {
                        name: variable_name.to_string(),
                        val: variable_value,
                    };
                    new_enum = VariableTypes::Integer(new_variable);
                }
                var_vec.push(new_enum);

            } else if line.starts_with("if") {
                
            
            } else if in_vec { 
                if line.contains("=") {
                    let arg = line.split_once("=").unwrap();
                    if !arg.1.contains("=") {
                        //println!("after equal:{}", arg.1);
                        //perform_operation(arg.1.into(), &mut var_vec, i);// variable is mutated by function
                        let variable_value_string = parse(&replace_varname_in_string(arg.1, &var_vec));
                        if variable_value_string.contains(".") {


                            match &var_vec[i] {
                                VariableTypes::Integer(variable) => {
                                    let variable_value: Option<f32> = Some(variable_value_string.parse().unwrap());//assigning new float to hold converterd type

                                    let new_variable: Variable<f32> = Variable {
                                        name: variable.name.clone(),
                                        val: variable_value,
                                    };
                                    let new_enum = VariableTypes::Float(new_variable);

                                    var_vec.remove(i);
                                    var_vec.push(new_enum);

                                }
                                VariableTypes::Float(variable) => {
                                    var_vec[i].mutate(variable_value_string);
                                }
                                VariableTypes::String(variable) => {

                                }
                                VariableTypes::Bool(variable) => {
                                    
                                }
                            }

                        } else {
                            println!("assigning int");

                            match &var_vec[i] {
                                VariableTypes::Integer(variable) => {
                                    var_vec[i].mutate(variable_value_string);
                                }
                                VariableTypes::Float(variable) => {
                                    let variable_value: Option<u32> = Some(variable_value_string.parse().unwrap());//assigning new float to hold converterd type

                                    let new_variable: Variable<u32> = Variable {
                                        name: variable.name.clone(),
                                        val: variable_value,
                                    };
                                    let new_enum = VariableTypes::Integer(new_variable);

                                    var_vec.remove(i);
                                    var_vec.push(new_enum);
                                }
                                VariableTypes::String(variable) => {

                                }
                                VariableTypes::Bool(variable) => {
                                    var_vec[i].mutate(variable_value_string);
                                }
                            }
                        }

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