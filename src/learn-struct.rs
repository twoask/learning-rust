struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn learn_struct() {
    let rect1 = Rectangle {
        width: 30, 
        height: 50
    };

    println!(
        "The area of the rectangle is {} square pixels",
        rect1.area());

    println!("rect1 is {rect1:#?}");

    let sq = Rectangle::square(10);

    println!(
        "The area of the square is {} square pixels",
        sq.area());

    println!("sq is {sq:#?}");
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn test_string() {
    // s1 is moved to s2, s1 is no longer valid
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{s2}, world!");

    let mut s = String::from("hello");
    s = String::from("ahoy");

    println!("{s}, world!");

    let s3 = String::from("hello");
    let s4 = s3.clone(); // s3 is still valid after the clone
    println!("s3 = {s3}, s4 = {s4}");

    let s = String::from("hello");
    takes_ownership(s); // s is moved into the function and is no longer valid here

    // println!("{s}"); // This would cause a compile-time error

    let x = 5;
    makes_copy(x); // x is copied into the function, x is still valid here
    println!("x = {x}");

    let (s5, len) = calculate_length(s4); // s4 is moved into the function, s5 is the returned ownership
    println!("The length of '{s5}' is {len}.");

    let mut user1 = build_user(String::from("someusername123"), String::from("email@example.com"));

    user1.email = String::from("anotheremail@example.com");

    println!("User1's email is {}, and active state is {}", user1.email, user1.active);

    let user2 = User {
        email: String::from("anewemail@example.com"),
        ..user1
    };

    println!("User2's email  is {}, and user1's email is {}", user2.email, user1.email);
}

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1
    }
}

fn takes_ownership(some_string: String) {
    println!("s = {some_string}");
}

fn makes_copy(some_integer: i32) {
    println!("i = {some_integer}");
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}