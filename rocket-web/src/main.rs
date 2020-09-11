// sean at shanghai
// learn from https://blog.logrocket.com/rust-web-apps-using-rocket-framework/
// for rust web app
// 20200909

#![feature(decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::http::{Header, Status};
use rocket::request::{Form, FromRequest, Outcome};
use rocket::response::content::Json;
use rocket::Request;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use serde::Serialize;
use std::net::IpAddr;

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

#[derive(Responder)]
struct MyResp<'my> {
    body: String,
    header: Header<'my>,
}

#[derive(Debug)]
struct NginxRealIp(IpAddr);

impl<'a, 'r> FromRequest<'a, 'r> for NginxRealIp {
    type Error = ();
    fn from_request(request: &Request<'r>) -> Outcome<Self, Self::Error> {
        match request.real_ip() {
            Some(ip) => Outcome::Success(NginxRealIp(ip)),
            _ => Outcome::Failure((Status::from_code(401).unwrap(), ())),
        }
    }
}

#[get("/ip")]
fn ip<'a>(ip: NginxRealIp) -> MyResp<'a> {
    let my = MyResp {
        body: format!("this is body"),
        header: Header::new("X-C", "this is header"),
    };
    println!("{:?}", ip);
    my
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
        .mount("/api/v1", routes![hello, new_book])
        .mount("/", StaticFiles::from("./root"))
        .mount("/", routes![ip])
        .attach(Template::fairing())
        .launch();
}
