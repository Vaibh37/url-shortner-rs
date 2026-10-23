use std::collections::HashMap;
use std::io::{self, Write};

fn ask(msg: &str) -> String {
    print!("{msg}");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    let mut links = HashMap::new();

    loop {
        let choice = ask("\n1. shorten\n2. find\n3. exit\n> ");

        match choice.as_str() {
            "1" => {
                let url = ask("url: ");
                let code = format!("url{}", links.len() + 1);

                links.insert(code.clone(), url);
                println!("code: {code}");
            }
            "2" => {
                let code = ask("code: ");

                match links.get(&code) {
                    Some(url) => println!("{url}"),
                    None => println!("not found"),
                }
            }
            "3" => break,
            _ => println!("invalid option"),
        }
    }
}
