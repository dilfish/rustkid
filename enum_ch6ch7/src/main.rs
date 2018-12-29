#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}


struct IpAddr {
    kind: IpAddrKind,
    address: String,
}


enum IpAddrNew {
    V4(String),
    V6(String),
}


// enum Option<T> {
//     Some(T),
//    None,
//     Nn,
// }

// match Some , _ = match all others


mod sound {
    pub fn guitar() {
        // Function body code goes here
	super::main();
    }
}


fn main() {
	let four = IpAddrKind::V4;
	let six = IpAddrKind::V6;
	println!("Hello, enum {:?}, {:?}!", four, six);
let home = IpAddr {
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"),
};

let loopback = IpAddr {
    kind: IpAddrKind::V6,
    address: String::from("::1"),
};
let home = IpAddrNew::V4(String::from("127.0.0.1"));
}
