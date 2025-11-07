// struct Recta {
//     height: f32,
//     width: f32,
// }

// fn main() {
//     let recta_a = Recta {
//         height: 20.1,
//         width: 10.2,
//     };
//     let area: f32 = get_area(&recta_a.height, &recta_a.width);
//     println!("{}", area);
// }

// fn get_area(a: &f32, b: &f32) -> f32 {
//     return a * b;
// }



struct Rectangle {
    height: f32,
    width: f32,
}

impl Rectangle {
    fn area(&self) -> f32 {
        return self.width * self.height;
    }
    
    fn print_something() {
        println!("Something that is static!");
    }
}

fn main() {
    let r: Rectangle = Rectangle { height: 10.0, width: 20.0 };
    println!("{}", r.area());
    
    // Call the static method
    Rectangle::print_something();
}