use std::fs;

fn main() {
    let contents = fs::read_to_string("a.txt");

    match contents {
        Ok(contents) => println!("{}", contents),
        Err(e) => println!("Error Reading file -> {}", e),
    }
}
