use std::io::{Write, stdin, stdout};
use std::process::exit;


#[derive(Debug)]
pub enum MetaCommandResult<T> {
    MetaCommandSuccess(T),
    MetaCommandUnrecognizedCommand,
}


#[derive(Debug)]
pub struct MetaCommand<'a>(&'a str);

impl<'a> MetaCommand<'a> {
    pub fn parse(&self) -> MetaCommandResult<&str> {
        match self.0 {
            ".exit" => exit(0),
            ".quit" => exit(0),
            _ => MetaCommandResult::MetaCommandUnrecognizedCommand,
        }
    }
}



fn main() {
    loop {
        let mut input = String::new();
        print!("# > ");
        let _ = stdout().flush();
        stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        match input.chars().nth(0) {
            Some(first_char) => {
                println!("{:?}", input);
                match first_char == '.' {
                    true => MetaCommand(&input).parse(),
                    false => MetaCommandResult::MetaCommandUnrecognizedCommand,
                };
            },
            None => println!("Unrecognized command '{}'.", input.trim()),
        }
    }
}
