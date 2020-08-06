// sean at shanghai

use std::fs;
use std::collections::HashMap;
use chrono::NaiveDate;

// read_file_ignore panic the program ignore the error
fn read_file_ignore() {
    let content = fs::read_to_string("./Cargo.toml").unwrap_or("no such file".to_string());
    println!("{}", content);
}

// read_file_expect let us write some message
fn read_file_expect() {
    let content = fs::read_to_string("./Cargo.toml").expect("Can't read main.rs");
    println!("{}", content);
}

// get_current_date return current time from a web api
fn get_current_date() -> Result<String, Box<dyn std::error::Error>> {
    // The ? operator is similar to unwrap but instead of panicking,
    // it propagates the error to the calling function. One thing to keep
    // in mind is that we can use the ? operator only for functions
    // that return a Result or Option type.
    let url = "https://postman-echo.com/time/object";
    let res = reqwest::blocking::get(url)?.json::<HashMap<String, i32>>()?;

    // let result = reqwest::blocking::get(url);

    // let response = match result {
       // Ok(res) => res,
       // Err(err) => return Err(err),
    // };

    // let body = response.json::<HashMap<String, i32>>();

    // let res = match body {
    // Ok(json) => json,
       // Err(err) => return Err(err),
    // };

    // let date = json["years"].to_string();
    let formatted_date = format!("{}-{}-{}", res["years"], res["months"] + 1, res["date"]);
    let parsed_date = NaiveDate::parse_from_str(formatted_date.as_str(), "%Y-%m-%d")?;
    let date = parsed_date.format("%Y %B %d").to_string();
    Ok(date)
}

fn show_closure() {
    // Increment via closures and functions.
    fn  function            (i: i32) -> i32 { i + 1 }
    // Closures are anonymous, here we are binding them to references
    // Annotation is identical to function annotation but is optional
    // as are the `{}` wrapping the body. These nameless functions
    // are assigned to appropriately named variables.
    let closure_annotated = |i: i32| -> i32 { i + 1 };
    let closure_inferred  = |i     |          i + 1  ;

    let i = 1;
    // Call the function and closures.
    println!("function: {}", function(i));
    println!("closure_annotated: {}", closure_annotated(i));
    println!("closure_inferred: {}", closure_inferred(i));

    // A closure taking no arguments which returns an `i32`.
    // The return type is inferred.
    let one = || 1;
    println!("closure returning one: {}", one());
}

fn main() {
    read_file_ignore();
    read_file_expect();
    match get_current_date() {
        Ok(date) => println!("We have time travlled to {}!", date),
        Err(e) => eprintln!("Oh ones, we do not know which era we are in!: {}", e),
    };
    show_closure();
}
