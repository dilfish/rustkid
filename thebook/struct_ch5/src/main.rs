#[derive(Debug)]
struct User {
    // if here username is &str, we could not
    // keep in track of the literal string when
    // it goes out of scope
    // for & is copied references
    // and references does not own the memory
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Color(i32, i32, i32);
#[derive(Debug)]
struct Point(i32, i32, i32);


// takes own of email and username
// return copied User
fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
    // User goes out of scope
}

fn previous_main() {
	let user1 = User {
	    email: String::from("someone@example.com"),
	    username: String::from("someusername123"),
	    active: true,
	    sign_in_count: 1,
	};
	let user2 = User {
 	   email: String::from("another@example.com"),
	    username: String::from("anotherusername567"),
   	 ..user1
	};
        let user3 = build_user(String::from("a"), String::from("b"));
	let black = Color(0, 0, 0);
	let origin = Point(0, 0, 0);
	println!("user1 {:?}, {:?}, {:?}", user1, user2, user3);
	println!("color {:?}, {:?}", black, origin);
}


fn area(w: u32, h: u32) -> u32 {
	w * h
}


fn area_t(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}


#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}


// reference of self
// does not own memory
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}


fn area_s(rect: &Rectangle) -> u32 {
	rect.width * rect.height
}

fn main() {
    previous_main();
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area_t(rect1)
    );
    let rect2 = Rectangle{width:30, height:50};
    println!(
	"aaaa {}",
	area_s(&rect2)
	);
    println!("debug {:?}", rect2);
    println!("impl {}", rect2.area());
}
