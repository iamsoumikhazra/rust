// Enums for dummies

enum Direction {
    North,
    South,
    East,
    West,
}

fn steer(dir: Direction) {
    if let Direction::North = dir {
        println!("User moves to the North");
    } else if let Direction::South = dir {
        println!("User moves to the South");
    } else if let Direction::East = dir {
        println!("User moves to the East");
    } else if let Direction::West = dir {
        println!("User moves to the West");
    }
}

fn main() {
    let user_direction: Direction = Direction::North;
    steer(user_direction);
}




// fn steer(dir: Direction) {
//     match dir {
//         Direction::North => println!("User moves to the North"),
//         Direction::South => println!("User moves to the South"),
//         Direction::East => println!("User moves to the East"),
//         Direction::West => println!("User moves to the West"),
//     }
// }

