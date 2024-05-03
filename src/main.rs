// fn main() {
    // println!("Hello, world!");

    //lets initialize vars
    // let x: i32 = -5;
    // let y: u32 = 26;
    // let z: f32 = 1000.002;

    // print!("x: {}, y: {}, z: {}", x, y, z);
    

    // boolean
        // let is_male = true;
        // let is_above_18 = true;
        
        // if is_male {
        //     println!("You are a male");
    
        // } else {
        //     println!("You are not a male");
        // }
    
        // if is_male && is_above_18 {
        //     print!("You are a legal male");
        // }

// String
    // let greeting = String::from("hello world");
    // println!("{}", greeting);

    // print!("{}", greeting.chars().nth(1000))


// for loops    
            // for i in 0..10{ //here 10 is not inclusive
            //     print!("{} ", i);
            // }
    
// functions
                // let a: i32 = 32;
                // let b: i32 = 23;
                // fn sum(a: i32, b: i32) -> i32{
                //     return a + b;
                // }
                // print!("{} is sum of {} and {} ", sum(a, b), a, b);


// Memory managment in rust - refer in ppt on slide 10
    // has its own ownership model its like c means manual but there are some rules so that there are no dangling pointer issues or any other memory issue for that matter

    // Not having a garbage collector is one of the key reasons rust is so fast
    // It achieves this using the
    // Mutability
    // Heap and memory
    // Ownership model
    // Borrowing and references
    // Lifetimes
    

// Mutability - by defaiult all the vars in rust are immutable. inherently thread-safe because if no thread can alter the data, then no synchronization is needed when data is accessed concurrently. Knowing that certain data will not change allows the compiler to optimize code better. 
        // let mut x: i32 = 1;
        // x = 2; // No error
        // println!("{}", x);


// Stack and Heaps
// Rust has clear rules about stack and heap data management:
// Stack: Fast allocation and deallocation. Rust uses the stack for most primitive data types and for data where the size is known at compile time (eg: numbers).
// Heap: Used for data that can grow at runtime, such as vectors or strings.



// Ownership - remeber copyign is very expensive



//structs
// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }


//     let user1 = User {
//         active: true,
//         username: String::from("someusername123"),
//         email: String::from("someone@example.com"),
//         sign_in_count: 1,
//     };
//     print!("User 1 username: {:?}", user1.username);
// }

//implement struct
    // struct Rect {
    //     width: u32,
    //     height: u32,
    // }


    // impl Rect {
    //     fn area(&self) -> u32 {
    //         self.width * self.height   //if we dont have semi-colon at the end then it is similar to return
    //     }
    // }

    // fn main() {
    //     let rect1 = Rect {
    //         width: 38,
    //         height: 43,
    //     };
    //     println!("The area of the rectangle is:  {} ", rect1.area());
    // }
    

// enums - makes code more stricter as you can play with only defined values
    // enum Direction {
    //     North,
    //     East,
    //     South,
    //     West,
    // }

    // fn main() {
    //     let my_direction = Direction::North;
    //     let new_direction = my_direction; // No error, because Direction is Copy
    //     move_around(new_direction); 
    // }

    // fn move_around(direction: Direction) {
    //     // implements logic to move a character around
    // }


// Pattern Matching

// Define an enum called Shape
// enum Shape {
//     Circle(f64),  // Variant with associated data (radius)
//     Square(f64),  // Variant with associated data (side length)
//     Rectangle(f64, f64),  // Variant with associated data (width, height)
// }

// // Function to calculate area based on the shape
// fn calculate_area(shape: Shape) -> f64 {
//     let ans = match shape {
//         Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
//         Shape::Square(side_length) => side_length * side_length,
//         Shape::Rectangle(width, height) => width * height,
//     };
//     return ans;
// }

// fn main() {
//     // Create instances of different shapes
//     let circle = Shape::Circle(5.0);
//     let square = Shape::Square(4.0);
//     let rectangle = Shape::Rectangle(3.0, 6.0);

//     // Calculate and print the areas
//     println!("Area of circle: {}", calculate_area(circle));
//     println!("Area of square: {}", calculate_area(square));
//     println!("Area of rectangle: {}", calculate_area(rectangle));
// }


// Error Handling - in js we used try catch block here we use a enum provided by rust. Its similar to try catch block

// use std::fs;
// fn main(){
//     let res = fs::read_to_string("example.txt");

//     match res{
//         Ok(content)=>{
//             print!("File content {}", content);
//         }
//         Err(err)=>{
//             print!("Error found: {}", err);
//         }
//     }
// }


// Unwraps
// Incase you are ok with runtime errors (crashing the process while it runs if an error happens), then you can unwrap a Result

// use std::fs;

// fn main() {
//     let greeting_file_result = fs::read_to_string("hello.txt");
//     print!("{}", greeting_file_result.unwrap());
// }


// Option enum - handle the concept of nullability in a safe and expressive way. Rust doesn't have null.

// fn find_first_a(s: String) -> Option<i32> {
//     for (index, character) in s.chars().enumerate() {
//         if character == 'z' {
//             return Some(index as i32);
//         }
//     }
//     return None;
// }

// fn main() {
//     let my_string = String::from("raman");
//     match find_first_a(my_string) {
//         Some(index) => println!("The letter 'a' is found at index: {}", index),
//         None => println!("The letter 'a' is not found in the string."),
//     }
// }


// Just like the nodejs ecosystem has npm, the rust ecosystem has cargo
// Cargo is a package manager for Rust
// use rand::{Rng, thread_rng};

// fn main() {
//     let mut rng = thread_rng();
//     let n: u32 = rng.gen();
//     println!("Random number: {}", n);
// }


// chrono
// use chrono::{Local, Utc};

// fn main() {
//     // Get the current date and time in UTC
//     let now = Utc::now();
//     println!("Current date and time in UTC: {}", now);

//     // Format the date and time
//     let formatted = now.format("%Y-%m-%d %H:%M:%S");
//     println!("Formatted date and time: {}", formatted);

//     // Get local time
//     let local = Local::now();
//     println!("Current date and time in local: {}", local);
// }
