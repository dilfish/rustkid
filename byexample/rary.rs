// sean at shanghai
// rustc --crate-type=lib rary.rs
// generate library.rlib

pub fn public_fn() {
    println!("this is a public function");
}

fn private_fn() {
    println!("this is a private func");
}

pub fn indirect_fn() {
    println!("called rary's indrect_fn");
    private_fn();
}
