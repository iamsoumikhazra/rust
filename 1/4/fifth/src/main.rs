// use std::fs;

// fn main() {
//     let contents = fs::read_to_string("a.txt");

//     match contents {
//         Ok(contents) => println!("{}", contents),
//         Err(e) => println!("Error Reading file -> {}", e),
//     }
// }

use std::io;

fn main() {
    println!("Please enter numbers you want to get summation of:");

    let mut x = String::new();
    io::stdin()
        .read_line(&mut x)
        .expect("Failed to read the value");

    let mut y = String::new();
    io::stdin()
        .read_line(&mut y)
        .expect("Failed to read the value");

    let x_num: u32 = x.trim().parse().expect("Please enter a valid number");
    let y_num: u32 = y.trim().parse().expect("Please enter a valid number");

    let mysum = sum(x_num, y_num);

    println!("The sum of two number is: {}", mysum);
}

fn sum(a: u32, b: u32) -> u32 {
    a + b
}
