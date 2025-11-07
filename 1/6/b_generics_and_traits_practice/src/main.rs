use std::ops::Mul;

fn main() {
    let problem = Calculator { number: 2 };
    println!("Square: {}", problem.square());
    println!("Double: {}", problem.double());
    problem.describe();

    let problem2 = Calculator { number: 2.5 };
    println!("Square: {}", problem2.square());
    println!("Double: {}", problem2.double());
    problem2.describe();
}

struct Calculator<T> {
    number: T,
}

trait Compute<T> {
    fn square(&self) -> T;
    fn double(&self) -> T;
}

trait Describe: std::fmt::Display {
    fn describe(&self);
}

impl<T: std::fmt::Display> std::fmt::Display for Calculator<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.number)
    }
}

impl<T: std::fmt::Display> Describe for Calculator<T> {
    fn describe(&self) {
        println!("This is a calculator holding number: {}", self.number);
    }
}

impl<T: Mul<Output = T> + Copy + From<i32>> Compute<T> for Calculator<T> {
    fn square(&self) -> T {
        self.number * self.number
    }
    fn double(&self) -> T {
        let two = 2.into();
        self.number * two
    }
}
