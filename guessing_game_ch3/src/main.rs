use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
	println!("guessing the number!");
	let secret_number = rand::thread_rng().gen_range(1, 101);
	// println!("the secret number is {}", secret_number);
	loop {
		println!("please input your guess.");
		let mut guess = String::new();
		io::stdin().read_line(&mut guess)
			.expect("failed to read line");
		println!("you enter {}", guess);
		let guess: u32 = match guess.trim().parse() {
			Ok(num) => num,
			Err(_) => continue,
		};
		println!("you guess {}", guess);
		match guess.cmp(&secret_number) {
			Ordering::Less => println!("Too small!"),
			Ordering::Greater => println!("Too big!"),
			Ordering::Equal => {
				println!("you win!");
				break;
			}
		}
	}
}
