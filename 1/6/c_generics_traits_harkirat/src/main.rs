fn main() {
    let rect = Rectangle { height: 5, width: 10 };
    println!("{} {}", rect.area(), rect.perimeter());

    let square = Square { side: 5 };
    println!("{} {}", square.area(), square.perimeter());

    let circle = Circle { radius: 5.0 };
    println!("{} {}", circle.area(), circle.perimeter());

    let circle2 = Circle { radius: 999999999 };
    println!("{} {}", circle2.area(), circle2.perimeter());
}

// shape area, perimeter for circle, square, rectangle

use std::{f64::consts::PI, ops::{Add, Mul}};

trait Shape {
    type Output;
    fn area(&self) -> Self::Output;
    fn perimeter(&self) -> Self::Output;
}

struct Rectangle<T> {
    height: T,
    width: T,
}

struct Circle<T> {
    radius: T,
}

struct Square<T> {
    side: T,
}

impl<T: Add<Output = T> + Mul<Output = T> + Copy + From<i32>> Shape for Rectangle<T> {
    type Output = T;

    fn area(&self) -> T {
        self.height * self.width
    }

    fn perimeter(&self) -> T {
        let two = T::from(2);
        two * (self.height + self.width)
    }
}

impl<T: Add<Output = T> + Mul<Output = T> + Copy + From<i32>> Shape for Square<T> {
    type Output = T;

    fn area(&self) -> T {
        self.side * self.side
    }

    fn perimeter(&self) -> T {
        let four = T::from(4);
        four * self.side
    }
}

impl<T: Copy + Into<f64>> Shape for Circle<T> {
    type Output = f64;

    fn area(&self) -> f64 {
        let radius_f = self.radius.into() as f64;
        PI * radius_f * radius_f
    }

    fn perimeter(&self) -> f64 {
        let radius_f = self.radius.into() as f64;
        2.0 * PI * radius_f
    }
}

