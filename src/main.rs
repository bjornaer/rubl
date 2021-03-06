// use ferris_says::say;
// use std::io::{stdout, BufWriter};

// fn main() {
//     let stdout = stdout();
//     let message = String::from("Rubl means RUst Based Language!\n(Until it becomes a RUbl Based Language!)");
//     let width = message.chars().count();

//     let mut writer = BufWriter::new(stdout.lock());
//     say(message.as_bytes(), width, &mut writer).unwrap();
// }

use std::fs;

use std::env;

#[derive(Debug)]
struct Lexer {
    contents: String,
}

impl Lexer {
    pub fn new(contents: String) -> Self {
        return Self { contents: contents };
    }

    pub fn lex(&self) {
        println!("{}",self.contents)
    }
}
fn main() {
    let maybe_file = env::args().nth(1);
    let file = if let Some(f) = maybe_file {
        f
    } else {
        panic!("Expected a file.")
    };

    let maybe_contents = fs::read_to_string(file);
    let contents = if maybe_contents.is_ok() {
        maybe_contents.unwrap()
    } else {
        panic!("Could Not Open file for reading.");
    };

    let lexer = Lexer::new(contents);

    lexer.lex()
}