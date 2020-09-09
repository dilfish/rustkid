// sean at shanghai
// learn from https://blog.logrocket.com/rust-web-apps-using-rocket-framework/
// for rust web app
// 20200909

#![feature(decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::request::Form;
use rocket::response::content::Json;
use rocket::Request;
use rocket_contrib::templates::Template;
use serde::Serialize;

#[derive(Serialize)]
struct Context {
    first_name: String,
    last_name: String,
}

#[derive(FromForm, Debug)]
struct Book {
    title: String,
    author: String,
    isbn: String,
}

// curl -X POST http://localhost:8000/api/v1/book -d 'title=title&isbn=isbn&author=author'
#[post("/book", data = "<book_form>")]
fn new_book(book_form: Form<Book>) -> String {
    let book: Book = book_form.into_inner();
    let mut dummy_db: Vec<Book> = Vec::new();
    dummy_db.push(book);
    format!("Book added successfully: {:?}", dummy_db)
}

#[get("/hello")]
fn hello() -> Json<&'static str> {
    Json(
        "{
            'status':'success',
            'message':'Hello API!'
        }",
    )
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Oh no, We could not find the request path '{}'", req.uri())
}

//     => Error: No matching routes for GET /favicon.ico image/avif.
#[get("/")]
fn index() -> Template {
    let context = Context {
        first_name: String::from("Sean"),
        last_name: String::from("Zhang"),
    };
    Template::render("home", context)
}

fn main() {
    rocket::ignite()
        .register(catchers![not_found])
        .mount("/", routes![index])
        .mount("/api/v1", routes![hello, new_book])
        .attach(Template::fairing())
        .launch();
}
