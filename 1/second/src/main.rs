// Demonstrating ownership, borrowing, and references in Rust.

fn main() {
    // ----- OWNERSHIP TRANSFER EXAMPLE -----
    // Ownership moves into `get_length()` and is returned back as a tuple.
    let my_str = String::from("Soumik");
    let (len, my_str) = get_length(my_str);
    println!("(1) Ownership example:");
    println!("Length: {}", len);
    println!("String: {}\n", my_str);

    // ----- BORROWING EXAMPLE -----
    // Using a reference so ownership is not moved.
    let my_str = String::from("Soumik");
    let str_len = get_len(&my_str); // Borrowed reference
    println!("(2) Borrowing example:");
    println!("Length (borrowed): {}", str_len);
    println!("String after borrow: {}\n", my_str);

    // Demonstrate immutable references
    println!("(3) Immutable reference example:");
    immutable_ref();

    // Demonstrate mutable references
    println!("\n(4) Mutable reference example:");
    mutable_ref();
}

// ---------------- OWNERSHIP FUNCTION ----------------
// Takes ownership of the String, returns both its length and itself.
fn get_length(a: String) -> (usize, String) {
    (a.len(), a)
}

// ---------------- IMMUTABLE BORROW FUNCTION ----------------
// Borrows the string; doesn't take ownership.
fn get_len(a: &String) -> usize {
    a.len()
}

// ---------------- IMMUTABLE REFERENCES ----------------
// Multiple immutable references are allowed at once.
fn immutable_ref() {
    let s1 = String::from("Soumik");
    let s2 = &s1;
    let s3 = &s1;
    let s4 = &s1;
    let s5 = &s1;

    println!("{}, {}, {}, {}, {}", s1, s2, s3, s4, s5);
}

// ---------------- MUTABLE REFERENCES ----------------
// Only one mutable reference is allowed at a time.
fn mutable_ref() {
    let mut s1 = String::from("Soumik");

    {
        // Limit the scope of the mutable reference
        let s2 = &mut s1;
        s2.push_str(" Hazra"); // Modify the string safely
        println!("Modified via mutable ref: {}", s2);
    } // s2 goes out of scope here

    // After `s2` is dropped, `s1` can be used again
    println!("After mutable ref is dropped: {}", s1);
}

//  From This we learn Either make all reference immutable OR make one reference mutable.