mod banner;
mod scrapper;

use std::io::{self, Write};

fn main() {
    print!("[+] Enter a keyword: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("there is issue in input");
    print!("[+] Searching....");
    eprint!("{:?}", scrapper::scrapper(input.trim().to_string()));
}
