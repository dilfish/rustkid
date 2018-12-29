// learn func

fn plus_one(x: i32) -> i32 {
	x + 1
}


fn takes_ownership(some :String) {
	println!("take {}", some);
}


fn give_ownership() -> String {
	let some = String::from("hello");
	some
}


fn takes_and_give_back(s: String) ->String {
	s
}


fn calc_len(s: &String) -> usize {
	s.len()
}


fn change(s: &mut String) {
	s.push_str("aaaa");
}


fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}


fn first_word_2(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}


fn main() {
	let x = plus_one(5);
	println!("x is {}", x);
	let s = String::from("hello");
	takes_ownership(s);
	// here we lost s
	let s1 = give_ownership();
	println!("s1 is {}", s1);
	let s2 = takes_and_give_back(s1);
	// here we give own from s1 to s2
	println!("s2 is {}", s2);
	let l = calc_len(&s2);
	println!("s 2 is {}, len {}", s2, l);
	let mut ms = String::from("mut.");
	change(&mut ms);
	let word = String::from("aword");
	let first = first_word(&word);
	println!("word is {}, first is {}", word, first);
	let first = first_word_2(&word);
	println!("word is {}, first is {}", word, first);
}
