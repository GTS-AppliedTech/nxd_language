use std::io::{self, Read};

use nxd_language::frontend::parser::parser::parse;

fn main() {
    let mut source = String::new();

    io::stdin().read_to_string(&mut source).unwrap();

    match parse(&source) {
        Ok(_) => println!("OK"),

        Err(error) => {
            println!(
                "{}|{}|{}: {}",
                error.span.line, error.span.column, error.code, error.message
            );
        }
    }
}
