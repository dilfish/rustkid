use std::thread;
use std::time::Duration;

fn main() {
    let expensive_closure = |num: u32| -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
};
    let g = expensive_closure(1);
    println!("Hello, world! {}", g);
}


// cargo build --release
// Cargo.toml:
// [profile.dev]
// opt-level = 0

// [profile.release]
// opt-level = 3


// doc comment use ///
// cargo doc --open
// cargo login abcdefghijklmnopqrstuvwxyz012345
// cargo publish
