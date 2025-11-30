
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



fn main() {
    let input = "20 + 20 -10";
    parse(input);
}