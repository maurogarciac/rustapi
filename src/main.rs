#[macro_use] extern crate rocket;

use rocket_dyn_templates::{Template, context};
use rocket::{serde::json::{serde_json::{json, Value}, Json}, http::Status};
use rocket::serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Test  {
    id: i32,
    title: String,
    body: String,
}

#[get("/")]
fn index() -> Template {
    Template::render("index", context!{
        title: "Test"
    })
}

#[get("/json")]
fn json() -> Value {
    json!({
        "message" : "how did you get here"
    })
}

#[get("/test")]
fn test() -> Json<Test> {
    Json(
        Test{
            id: 1,
            title: "Test 1".to_string(),
            body: "Lorem ipsum etc...".to_string(),
        }
    )
}
#[get("/posts/<name>")]
fn get_blog_posts(name:&str) -> Result<Template, Status> {
    let s_name : String = name.to_string(); 
    let template : Template = Template::render(s_name, context! {
        title: format!("Post {}", name)
    });
    return Ok(template);
    /*
    if let Some(template) = template {
        return Ok(template);
    }else {
        return Err(Status::NotFound);
    }
    */
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, json, test, get_blog_posts])
        .attach(Template::fairing())
}


