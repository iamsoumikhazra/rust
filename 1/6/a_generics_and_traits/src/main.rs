use std::ops::Mul;

fn main() {
    println!("Hello, world!");
    
    // Using specialized functions for specific types
    println!("{}", sum_u32(10, 20));
    println!("{}", sum_f32(10.0, 20.0));
    
    // Using generic multiplication function
    println!("{}", multiplication(10, 10));        // i32 * i32
    println!("{}", multiplication(2.5, 2.0));      // f64 * f64 - Note: both same type
    println!("{}", multiplication(2.5, 2f32));      // f64 * f64 - Note: both same type
    
    
    // Using generic display function
    display_elements("S", "H");
}

// Problem: We have to define same function for different data types multiple times
// This violates the DRY (Don't Repeat Yourself) principle

// Specialized function for u32
fn sum_u32(a: u32, b: u32) -> u32 {
    return a + b;
}

// Specialized function for f32  
fn sum_f32(a: f32, b: f32) -> f32 {
    return a + b;
}

// Generic multiplication function
// T: Mul<X, Output = T> means:
// - Type T can multiply with type X
// - The result of multiplication is type T
// - This allows different input types but output same as first operand
fn multiplication<T: Mul<X, Output = T>, X>(a: T, b: X) -> T {
    return a * b;
}

// Alternative generic multiplication (more common approach)
// This requires both operands to be the same type
// fn multiplication<T: Mul<Output = T>>(a: T, b: T) -> T {
//     a * b
// }

// Generic function to display elements
// T: std::fmt::Display means type T must be printable
// Both parameters must be the same type
fn display_elements<T: std::fmt::Display>(a: T, b: T) {
    println!("{}", a);
    println!("{}", b);
}

// ===== IMPORTANT NOTES =====
// 
// 1. The multiplication(2.5, 2) call was failing because:
//    - 2.5 is f64 (floating point)
//    - 2 is {integer} (compiler doesn't know which integer type)
//    - f64 doesn't implement Mul<{integer}> trait by default
//
// 2. Solutions:
//    a) Use same types: multiplication(2.5, 2.0)
//    b) Specify integer type: multiplication(2.5, 2i32)
//    c) Use trait bounds that work with specific types
//
// 3. Generic functions help avoid code duplication while maintaining type safety
//
// 4. Common trait bounds:
//    - std::fmt::Display: for types that can be printed
//    - std::ops::Add: for types that support addition
//    - std::ops::Mul: for types that support multiplication
//    - Clone: for types that can be cloned
//    - Copy: for types that can be copied (implies Clone)
// 
// 
// 