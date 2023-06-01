#[macro_use] extern crate rocket;
use rocket_dyn_templates::{Template, context};

#[get("/")]
fn hello() -> Template {
    Template::render("index", context!{
        title: "Test"
    })
}

#[get("/hi")]
fn test() -> &'static str {
    "how did you get here"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello, test])
    .attach(Template::fairing())
}
