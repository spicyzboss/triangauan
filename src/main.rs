use std::io::{stdin, stdout, Write};
mod triangle;

fn main() {
    print!("Enter triangle size: ");
    let _ = stdout().flush();

    let mut raw: String = String::new();
    stdin().read_line(&mut raw).unwrap();

    match raw.trim().parse::<u64>() {
        Ok(x) => triangle::print_triangle(x),
        Err(_) => println!("Unsigned integer only!"),
    };
}
