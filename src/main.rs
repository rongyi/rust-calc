use std::io::{self, Write};
mod rymath;

fn main() {
    println!("ry calc");
    let mut input = String::new();
    loop {
        print!(">");
        io::stdout().flush().unwrap();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                println!("todo: {}", input);
                input.clear();
            }
            Err(err) => println!("error: {}", err),
        }
    }
}
