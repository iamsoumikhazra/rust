// fn main() {
//     println!("Hello, world!");
//     let a:u32 = 10;
//     let b:u32 = 20;

//     let my_str = String::from("Soumik");
//     let add = sum(a,b);
//     let my_len =getLength(my_str);
//     println!("{}",add);
//     println!("{}",my_len);
//     // now suppose you want to use a and b you can
//     println!("{}",a);
//     //but suppose you want to use string
//     // println!("{}",my_str);  // here you cant because the ownership of my_str is gone to my_len so you cant use my_str.

// }

// fn sum(a:u32,b:u32)->u32{
//     return a+b ;
// }

// fn getLength(s:String)->usize{
//     return s.len();
// }

// //mutable reference And immutable references

// use std::cell::RefCell;

// fn main() {
//        let x = RefCell::new(5);

//     //    let b = x.borrow_mut(); // Line Z
//        let a = x.borrow();
    


//     // println!("{}", b);
//     println!("{}", a);
//     // println!("{}, {}", a, b);
// }


// fn main() {
//     let s = String::from("Soumik");
//     // let r = &s;
//     // println!("{}", r);
//     println!("{}", s);
// }

// fn main() {
//     let r = "Soumik";
//     println!("{}", r);
// }

// fn main() {
//     let s = "Soumik";
//     let name = s.to_string();

//     println!("{}",name);
// }

// fn main() {
//     let name = String::from("Soumik");
//     let modified = add_title(&name);
//     println!("Original: {name}");
//     println!("Modified: {modified}");
// }

// fn add_title(s: &String) -> String {
//     format!("Mr. {}", s)
// }

// fn main(){
//     // Why does this work?
// let s1 = String::from("hello");
// let s2 = &s1;           // s2 is a reference
// let s3 = &s1[0..2];     // s3 is also a reference (&str)
// // But this doesn't:
// // let s4 = s1[0];      // ❌ Why?

// println!("{}",s3)
// }

// 🎓 Homework: Final Session 1 Task

// fn main(){
//     let my_str_one = String::from("My First String");

//     // let my_str_one_length = getLength(my_str_one);
//    // println!("The size of the string -> {} is {}",my_str_one,my_str_one_length);  //As you can see i gave the ownership of "My First String" to getLength so my_str_one become invalid. 

//     let my_str_one_length = getLengthByRef(&my_str_one);
//     println!("The size of the string -> {} is {}",my_str_one,my_str_one_length);    // This one will run as i let borrow the value of my_str_one to getLengthByRef.
    

//     let my_str_two = "My Second String";
//     let my_str_two_length = getLengthFromRefStr(&my_str_two);
//      println!("The size of the string -> {} is {}",my_str_two,my_str_two_length);

// }


// // fn getLength(s:String)->usize{
// //     return s.len();
// // }

// fn getLengthByRef(s:&String)->usize{
//     return s.len();
// }


// fn getLengthFromRefStr(s:&str)->usize{
//     return s.len();
// }



// using String is better because it can be change as it stored in heap memory. but str you cant change as it lives is static memory inside program. String is better but you cant use this solana program as bpf doesn't support heap memory.


// struct Book {
//     title: String,
//     pages: u32,
// }

// fn main() {
//     let book1 = Book {
//         title: String::from("Rust Book"),
//         pages: 300,
//     };
//     // let book2 = book1;
//     // we can do this using different method like 1. removing let book2 = book1;
//     // let book2 = &book1; // 2. by using reference of book1 we can fix this 
//     println!("{}", book1.title); // What happens?
// }


// struct User {
//     name: String,
//     age: u32,
// }

// fn main() {
//     let user1 = User { name: String::from("Soumik"), age: 20 };         //--> name is stored in a heap and age is stored in the stack and the reference of the heap address is stored in the stack also which says the ownership of Soumik is goes to user1's name. 
//     let user2 = user1;  // What happens to name? What happens to age?   //--> now user2 will get the ownership of user1's name and user1's age. so user1 become invalid. 
    
//     // Which of these lines compile?
//     // println!("{}", user1.name);     // so this line will not work
//     // println!("{}", user1.age);      // same this line also not work
// }

// struct Database {
//     connection_string: String,
//     port: u16,
    
// }

// fn connect(db: &Database) -> String {
//     format!("Connecting to {}:{}", db.connection_string, db.port)
// }

// fn main() {
//     let db = Database {
//         connection_string: String::from("localhost"),
//         port: 5432,
//     };
    
//     let conn_info = connect(&db);
//     println!("DB config: {}:{}", db.connection_string, db.port);
//     println!("{}", conn_info);
// }

// 1. Yes
// 2. Because Database struct can store a connection_string(which is a String) and a port(which is a u16), now we have a instance of Database called db which stored localhost as connection_string and 5432 as port. so now the ownership of localhost and 5432 is under db variable. And now we 
// using and reference of the db (&db) for conn_info.  so the ownership of localhost and 5432 is still under db. so now if we run  println!("DB config: {}:{}", db.connection_string, db.port); it will run perfectly and also println!("{}", conn_info); line also will work because it just borrowed the value of db not taking the ownership.
// 3. if we change connect(&db) to connect(db) then the it will ask me change the fn connect(db: &Database) to fn connect(db:Database) after we did that we have issues with  println!("DB config: {}:{}", db.connection_string, db.port); as the db's ownership already transferred to conn_info.



// Build a Library system that demonstrates:
// 1. Structs with both owned (String) and Copy (u32) fields
// 2. Methods that borrow (&self) vs take ownership (self)
// 3. The difference between moved and borrowed structs

// Requirements:
// - Book { title: String, year: u32 }
// - Library { books: Vec<Book> }
// - Implement add_book(&mut self, book: Book)
// - Implement get_book_count(&self) -> usize  
// - Demonstrate what happens when you try to:
//   let book = library.books[0];  // (Hint: this moves!)
// - Fix it by using a reference instead

// HINT: For get_book_count, think: does the function need to own the Library?


struct Book {
    title: String,
    year: u32,
}

struct Library {
    books: Vec<Book>,
}

impl Library {
    // &mut self because Library stays, only books vector changes
    fn add_book(&mut self, book: Book) {
        return self.books.push(book);
    }
    fn get_book_count(&self)-> usize {
        return self.books.len();
    }
 fn find_books_by_year(&self, year: u32) -> Vec<&Book> {
    let mut results: Vec<&Book> = Vec::new();

    for book in &self.books {
        if book.year == year {
            results.push(book);
        }
    }

    results
}

}

fn main() {
    let book1 = Book {
        title: String::from("my_book"),
        year: 2001,
    };
    let book2 = Book {
        title: String::from("my_book_2"),
        year: 2002,
    };
    let book3 = Book {
        title: String::from("my_book_3"),
        year: 2003,
    };

    // Create library with an empty book vector
    let mut library = Library {
        books: Vec::new(),
    };

    library.add_book(book1);
    library.add_book(book2);
    library.add_book(book3);


    let book = &library.books[0];
}
