#![feature(decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
use std::{borrow::Borrow, path::{PathBuf,Path}, string};
use rocket::response::content;
use rocket::{http::RawStr, response::NamedFile};
use rocket_contrib::{templates::Template,serve::StaticFiles};
use rocket::request::Form;
use serde::Serialize;
#[get("/hello/<name>/<age>/<cool>")]
fn hello(name:String,age:u8,cool:bool) -> String{
    if cool{
        format!("You're a cool {} year old, {}!", age, name)
    }
    else{
        format!("{}, we need to talk about your coolness.", name)
    }
}
#[get("/page/<path..>")]
fn get_page(path:PathBuf) -> String{
    format!("ans:{}",path.as_os_str().to_str().unwrap())
}

#[get("/<file..>")]
fn files(file:PathBuf) -> Option<NamedFile>{
    NamedFile::open(Path::new("static/").join(file)).ok()
}

#[get("/user/<id>")]
fn user(id:usize) -> &'static str{
    "unsigned"
}
#[get("/user/<id>",rank = 2)]
fn user_int(id:isize) -> &'static str{
    "signed"
}
#[get("/user/<id>",rank = 3)]
fn user_str(id:&RawStr) -> &'static str{
    "string"
}
#[get("/hello?wave&<name>")]
fn hello1(name:Option<String>) -> String{
    name.map(|name| format!("Hello,{}",name)).unwrap_or_else(|| "Hello!".into())
}
#[derive(FromForm,Serialize)]
struct User{
    name:String,
    account:usize
}
#[get("/item?<id>&<user..>")]
fn item(id:usize,user:Form<User>) -> String{
    format!("User\nName : {}\nAccount : {}",user.name,user.account)
}
#[get("/")]
fn index() -> Template{
    let u = User{
        name:String::from("John"),
        account:20210129
    };
    Template::render("index", u)
}
fn main() {
    rocket::ignite()
    .mount("/", routes![hello,get_page,user,user_int,user_str,hello1,item,index])
    .mount("/static", StaticFiles::from("static"))
    .attach(Template::fairing())
    .launch();
}
