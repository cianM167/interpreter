use regex::Regex;

fn parse(string: &str) -> String {
    println!("in string:{}", string);
    let mut res: String = string.to_string().replace(" ", "");
    let re = Regex::new(r"\((.*?)\)").unwrap();//very naiive
    let mut out: String = " ".to_string();

    let mut matches = tokenize(&res);

    for mat in matches {
        if mat.contains("(") {
            out = parse(&mat);
        } else {
            out = mat.into();
        }
    }

    /*
    while !mat.is_empty() {
        let matched_portion = mat[0];
        let eq = &string[matched_portion.start()..matched_portion.end()];
        let mut parsed = rem_first_and_last(eq).to_owned();
        println!("{}", parsed);

        if parsed.contains("(") {
            parsed = parsed + ")";
            let to_add = parse(&parsed);
            res = re.replace(&res, to_add).to_string();
            let mut open_bracket = false;

            for mut c in res.chars() {
                if c == '(' {
                    open_bracket = true;
                } else if c == ')' && open_bracket {
                    open_bracket = false;
                } else if c == ')' {
                    //c = '';
                }
            }
        } else {
            parsed = parsed.replace(" ", "");
            println!("parsed:{}", parsed);
            if parsed.contains("+") {
                let chunks = parsed.split_once("+").unwrap();

                println!("chunk 0:{}", chunks.0);
                let var1: u32 = chunks.0.parse().unwrap();
                println!("chunk 1:{}", chunks.1);
                let var2: u32 = chunks.1.parse().unwrap();

                parsed = (var1 + var2).to_string();
            }

            res = re.replace(&res, parsed).to_string();
        }

        println!("res:{}", res);
        mat = re.find(&res); 
    }
    */
    
    println!("returning:{}", out);
    return out;
}

fn rem_first_and_last(value: &str) -> &str {
    let mut chars = value.chars();
    chars.next();
    chars.next_back();
    chars.as_str()
}

fn tokenize(value: &str) -> Vec<&str> {
    let mut depth = 0;
    let mut tok_start = 0;
    let mut tok_end = 0;
    let mut token_vector: Vec<&str> = vec![];
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

                token_vector.push(&value[tok_start..tok_end]);
                println!("{:?}", token_vector);
            } 

        }
    }

    return token_vector;
}

fn first_token(value: &str) -> (usize, usize){
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

                return (tok_start, tok_end);
            } 

        }
    }

    return (0,0); //error out
}



fn main() {
    let input = "(10 + (21)) + 10";
    let (start, end) = first_token(input);
    println!("{:?}", input.replace(&input[start..end], ""));
    let out = parse(input);

    //println!("{:?}", tokenize(input));

    //println!("{}", out);
}
