struct User {
	username: String,
	email: String,
	sign_in_count: u64,
	active: bool
}

// Tuple Structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

#[derive(Debug)]
struct Rectangle {
     width: u32,
     height: u32,
}

impl Rectangle {
        fn area(&self) -> u32 {
                self.width * self.height
        }
        fn update_width(&mut self, update_to: u32) ->() {
                self.width += update_to;
        }
        fn can_hold(&self, other: &Rectangle) -> bool {
                self.width > other.width && self.height > other.height
        }
}

//  field init shorthand syntax
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}


fn main() {
    let user1 = build_user(String::from("debjani"), String::from("banerjee"));
    // struct update syntax
    let user2 = User {
       email: String::from("rajarshi@cool2.com"),
       username: String::from("roy"),
       ..user1
    };
    println!("Hello, world!");
    let black = Color(0,0,0);
    let origin = Point(0,0,0); 
    println!("{} * {} = {}", 30, 40, area(30, 40));
    
    let mut rect2 = Rectangle { width: 30, height: 40 };
    println!("{} * {} = {} ", rect2.width, rect2.height, rect2.area());
    rect2.update_width(10); 
    println!("{} * {} = {} ", rect2.width, rect2.height, rect2.area());
    
    println!("rect is {:#?}", rect2);

    let rect3 = Rectangle { width: 10, height: 30 };
    println!("rect2 can hold rect3, {}", rect2.can_hold(&rect3));

}

fn area(width: u32, height: u32) -> u32 {
	width * height
}
