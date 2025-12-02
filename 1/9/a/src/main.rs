fn main() {
    println!("Hello, world!");
    let my_sum = sum(10,12);
    let my_substraction = substraction(15,12);
    println!("{} {}",my_sum,my_substraction);
}


fn sum(a:u8,b:u8)->u8{
    return a+b;
}

fn substraction(a:u8,b:u8)->u8{
    return a-b;
}   
