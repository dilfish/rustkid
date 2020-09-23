// sean at shanghai
// learn rust for lifetime
// 20200923

// ref: http://troubles.md/why-phantomdata/

// XXX this file could not compile
// if we added unsafe prefix to take_static and all other needed
// it will compile but give us a bad result
// OUTPUT:
// we give msg to GMSG: lifetime 'a string
// [string 'a goes out of scope]
// global msg is lifetime 'a string

static mut GMSG: &'static str = "lifetime 'static string";

fn take_static(msg: &'static str) {
    println!("we give msg to GMSG: {}", msg);
    GMSG = msg;
}

struct PointlessWrapper<T>(T, fn(T) -> ());

fn expect_a<'a>(t: PointlessWrapper<&'a str>, m: &'a str) {
    t.1(m);
}

fn main() {
    {
        let a = "lifetime 'a string";
        let x = PointlessWrapper(a, take_static);
        expect_a(x, a);
    }
    // println!("here string a goes out of life time {}", a);
    println!("global msg is {}", GMSG);
}
