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
        println!("guitar");
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
    println!("kind is{:?}, address is {:?}", home.kind, home.address);

    let _loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    let _home = IpAddrNew::V4(String::from("127.0.0.1"));
    let _home_v6 = IpAddrNew::V6(String::from("1::1"));
    sound::guitar();
}
