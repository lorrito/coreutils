use std::{env, fs};

fn filter(text: &str, filter: &str) {
    for line in text.lines() {
        if line.contains(filter) {
            println!("{}", line);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let text = fs::read_to_string(&args[1]);
    match text {
        Ok(text_content) => {
            filter(&text_content, &args[2])
        },
        Err(e) => eprintln!("{}", e),
    }
}
