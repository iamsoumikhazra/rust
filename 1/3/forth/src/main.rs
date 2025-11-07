// Enums for dummies

// enum Direction {
//     North,
//     South,
//     East,
//     West,
//     SouthEast,
//     NorthEast,
//     SouthWest,
//     NorthWest
// }

// fn steer(dir: Direction) {
//     match dir {
//         Direction::North => println!("User moves to the North"),
//         Direction::South => println!("User moves to the South"),
//         Direction::East => println!("User moves to the East"),
//         Direction::West => println!("User moves to the West"),
//         _ => println!("User moves to another direction"), // SouthEast falls here
//     }
// }

// fn steer(dir: Direction) {
//     match dir {
//         Direction::North => println!("User moves to the North"),
//         Direction::South => println!("User moves to the South"),
//         Direction::East => println!("User moves to the East"),
//         Direction::West => println!("User moves to the West"),
//     }
// }

// fn main() {
//     let user_direction: Direction = Direction::SouthEast;
//     steer(user_direction);
// }

// use std::f32::consts::PI;
// enum Shape{
//     Square(f32),
//     Circle(f32),
//     Rectangle(f32,f32)
// }

// fn calculate_properties(s:Shape){
//     match s {
//         Shape::Square(side)=>{
//             let perimeter = 4.0 * side;
//             let area = side * side;

//             println!("The perimeter of the Square is {} and area is {}",perimeter,area);
//         }
//         Shape::Rectangle(length,width   )=>{
//             let perimeter = 2.0 * (length+width);
//             let area = length * width;

//             println!("The perimeter of the Rectangle is {} and area is {}",perimeter,area);
//         }
//         Shape::Circle(radius)=>{
//             let perimeter = 2.0 * PI * radius;
//             let area = PI * radius * radius;

//             println!("The perimeter of the Circle is {} and area is {}",perimeter,area);
//         }
//     }
// }

// fn main(){
//     let my_shape_1 = Shape::Square(10.22);
//     let my_shape_2 = Shape::Circle(10.22);
//     let my_shape_3 = Shape::Rectangle(10.22,10.32);

//     calculate_properties(my_shape_1);
//     calculate_properties(my_shape_2);
//     calculate_properties(my_shape_3);
// }

// better way to write this
// use std::f32::consts::PI;

// #[derive(Debug)]
// enum Shape {
//     Square(f32),
//     Circle(f32),
//     Rectangle(f32, f32),
// }

// fn calculate_properties(s: &Shape) -> (&Shape, f32, f32) {
//     match s {
//         Shape::Square(side) => {
//             let perimeter = 4.0 * side;
//             let area = side * side;
//             (s, perimeter, area)
//         }
//         Shape::Circle(radius) => {
//             let perimeter = 2.0 * PI * radius;
//             let area = PI * radius * radius;
//             (s, perimeter, area)
//         }
//         Shape::Rectangle(height, width) => {
//             let perimeter = 2.0 * (height + width);
//             let area = height * width;
//             (s, perimeter, area)
//         }
//     }
// }

// fn main() {
//     let my_shape_1 = Shape::Square(10.22);
//     let my_shape_2 = Shape::Circle(10.22);
//     let my_shape_3 = Shape::Rectangle(10.22, 10.32);

//     let (s1, p1, a1) = calculate_properties(&my_shape_1);
//     println!("{:?} => Perimeter: {p1}, Area: {a1}", s1);

//     let (s2, p2, a2) = calculate_properties(&my_shape_2);
//     println!("{:?} => Perimeter: {p2}, Area: {a2}", s2);

//     let (s3, p3, a3) = calculate_properties(&my_shape_3);
//     println!("{:?} => Perimeter: {p3}, Area: {a3}", s3);
// }

// you also can use impl in top of enums

// use std::f32::consts::PI;

// #[derive(Debug)]
// enum Shape {
//     Square(f32),
//     Circle(f32),
//     Rectangle(f32, f32),
// }

// impl Shape {
//     fn calculate_properties(&self) -> (&Self, f32, f32) {
//         match self {
//             Shape::Square(side) => {
//                 let perimeter = 4.0 * side;
//                 let area = side * side;
//                 (self, perimeter, area)
//             }
//             Shape::Circle(radius) => {
//                 let perimeter = 2.0 * PI * radius;
//                 let area = PI * radius * radius;
//                 (self, perimeter, area)
//             }
//             Shape::Rectangle(height, width) => {
//                 let perimeter = 2.0 * (height + width);
//                 let area = height * width;
//                 (self, perimeter, area)
//             }
//         }
//     }
// }

// fn main() {
//     let my_shape_1 = Shape::Square(10.22);
//     let my_shape_2 = Shape::Circle(10.22);
//     let my_shape_3 = Shape::Rectangle(10.22, 10.32);

//     let (s1, p1, a1) = my_shape_1.calculate_properties();
//     println!("{:?} => Perimeter: {p1}, Area: {a1}", s1);

//     let (s2, p2, a2) = my_shape_2.calculate_properties();
//     println!("{:?} => Perimeter: {p2}, Area: {a2}", s2);

//     let (s3, p3, a3) = my_shape_3.calculate_properties();
//     println!("{:?} => Perimeter: {p3}, Area: {a3}", s3);
// }

// Using struct in top of all this
use std::f32::consts::PI;

enum Shape {
    Square(f32),
    Circle(f32),
    Rectangle(f32, f32),
}

struct ShapeProperties {
    name: String,
    perimeter: f32,
    area: f32,
}

impl Shape {
    fn calculate(&self) -> ShapeProperties {
        match self {
            Shape::Square(side) => ShapeProperties {
                name: "Square".to_string(),
                perimeter: 4.0 * side,
                area: side * side,
            },
            Shape::Circle(radius) => ShapeProperties {
                name: "Circle".to_string(),
                perimeter: 2.0 * PI * radius,
                area: PI * radius * radius,
            },
            Shape::Rectangle(h, w) => ShapeProperties {
                name: "Rectangle".to_string(),
                perimeter: 2.0 * (h + w),
                area: h * w,
            },
        }
    }
}

fn main() {
    let shapes = [
        Shape::Square(10.22),
        Shape::Circle(10.22),
        Shape::Rectangle(10.22, 10.32),
    ];

    for shape in shapes {
        let props = shape.calculate();
        println!("{} => Perimeter: {:.2}, Area: {:.2}",props.name, props.perimeter, props.area);
    }
}
