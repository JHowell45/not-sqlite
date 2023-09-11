use std::io::{Write, stdin, stdout};
use std::process::exit;
use std::fmt;

#[derive(Debug)]
pub enum MetaCommandResult<T, E> {
    MetaCommandSuccess(T),
    MetaCommandUnrecognizedCommand(E),
}

#[derive(Debug, Clone)]
pub struct MetaCommandUnrecognizedCommand<'a>(&'a str);

impl fmt::Display for MetaCommandUnrecognizedCommand<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", format!("Unrecognized command '{}'! Type '.q' to exit the REPL.", self.0))
    }
}


#[derive(Debug)]
pub struct MetaCommand<'a>(&'a str);

impl<'a> MetaCommand<'a> {
    pub fn parse(&self) -> MetaCommandResult<&str, &str> {
        match self.0 {
            ".q" => exit(0),
            _ => MetaCommandResult::MetaCommandUnrecognizedCommand(self.0),
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
                let _ = match first_char == '.' {
                    true => match MetaCommand(&input).parse() {
                        MetaCommandResult::MetaCommandSuccess(_) => continue,
                        MetaCommandResult::MetaCommandUnrecognizedCommand(e) => {
                            println!("{}", MetaCommandUnrecognizedCommand(e));
                            let _ = stdout().flush();
                            continue;
                        },
                    },
                    false => {
                        println!("{}", MetaCommandUnrecognizedCommand(input.trim()));
                        let _ = stdout().flush();
                        continue;
                    },
                };
            },
            None => println!("{}", MetaCommandUnrecognizedCommand(input.trim())),
        }
    }
}
