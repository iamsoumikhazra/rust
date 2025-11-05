fn main() {   //entry point function
    println!("Hello, world!"); //macro

    let a:u32 = 25;
    let b:u32 = 25;

    let ans:u32 = sum(a,b);

    println!("The sum of {} and {} is {}",a, b, ans);


    let my_even: bool = is_even(ans);

    if my_even {
        println!("The number is even")
    }
    else {
        println!("The number is odd")
    }

    my_name();
    prices();
}

fn sum(a:u32, b:u32) -> u32{
    return a+b;
}

fn is_even(a: u32) -> bool{
    if a % 2 == 0 {
        return true;
    }
    else {
        return false;
    }
}



fn my_name(){
    let name: String = String::from("Soumik");
    println!("{}",name)
}

fn prices(){
    let price:Vec<i32> = vec![1,2,3];
    println!("{:?}", price);
}



// in stack memory the value of the variable literally copied but in the case of heap the address get copied.

fn mem(){

let x  = 5; // i32, stored on the stack, implements Copy.
let y = x; // x is copied, so both x and y are valid.

let s1 = String::from("hello"); // String is stored on the heap.
let s2 = s1; // s1 is moved to s2, so s1 is no longer valid.
}

fn mem_safe() {
    let a = 5; // stack: a = 5
    let b = a; // stack: b = 5 (copy)

    let s = String::from("hello"); // heap: "hello", stack: s (pointer to heap)
    let t = s; // moves the pointer from s to t, s is no longer valid.

    // Using `s` here would cause a compile-time error.
    println!("{}", t); // okay

    // When `mem_safe` ends, `a` and `b` are popped off the stack, and the heap memory for `t` (formerly `s`) is freed.
}


fn call_stack(){
    let a=21;
    let b=22;

    let z = sum(a,b);
    println!("{}", z)
}

