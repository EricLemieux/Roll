use roll::roll;
use std::env::args;

fn main() {
    let command = args().nth(1).expect("Expected pattern");

    match roll(command.as_str()) {
        Ok(value) => {
            println!("{}", value)
        }
        Err(err) => {
            eprintln!("{}", err)
        }
    }
}
