use std::io::{self, Write};
mod rymath;
use rymath::node;
use rymath::parser::Parser;

fn eval(expr: String) -> Result<f64, String> {
    let expr = expr.split_whitespace().collect::<String>();
    let mut p = Parser::new(&expr)?;
    let ast = p.parse()?;
    println!("ast: {:?}", ast);

    Ok(node::eval(ast)?)
}

fn main() {
    println!("ry calc");
    loop {
        let mut input = String::new();
        print!(">");
        io::stdout().flush().unwrap();
        match io::stdin().read_line(&mut input) {
            Ok(_) => match eval(input) {
                Ok(val) => println!("{}", val),
                Err(s) => println!("{}", s),
            },
            Err(err) => println!("error: {}", err),
        }
    }
}
