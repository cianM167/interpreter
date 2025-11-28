use regex::Regex;

fn parse(string: &str) -> String {
    println!("in string:{}", string);
    let mut res: String = "".into();
    let re = Regex::new(r"\((.*?)\)").unwrap();//very naiive

    for mat in re.find_iter(string) {
        let eq = &string[mat.start()..mat.end()];
        println!("eq:{}", eq);
        let mut parsed = rem_first_and_last(eq).to_owned();
        if parsed.contains("(") {
            parsed = parsed + ")";
            println!("{}", parsed);
            let to_add = parse(&parsed);
            println!("{}", to_add);
            parsed = re.replace(&parsed, to_add).to_string();
  
        }

            println!("res:{}", parsed);
            parsed = parsed.replace(" ", "");

            if parsed.contains("+") {
                let chunks = parsed.split_once("+").unwrap();
                println!("chunk:{}",chunks.0);
                let num1: u32 = (chunks.0).parse().unwrap();
                let num2: u32 = (chunks.1).parse().unwrap();
                
                parsed = (num1 + num2).to_string();
                res = re.replace(&parsed, &parsed).to_string();
                println!("sum:{}", res);
            } else {
                println!("error");
            }
    }
    


    println!("returning:{}", res);
    return res;
}

fn rem_first_and_last(value: &str) -> &str {
    let mut chars = value.chars();
    chars.next();
    chars.next_back();
    chars.as_str()
}

fn main() {
    let input = "(1 + 10) + (10 + (10))";
    parse(input);
}
