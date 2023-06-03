pub mod templates;

#[macro_use] extern crate rocket;

use rocket_dyn_templates::{Template, context};
use rocket::{serde::json::{serde_json::{json, Value}, Json}, http::Status};
use rocket::serde::{Serialize, Deserialize};

use templates::validate;


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
//define something like this 
// '/<year>/<month>/<day>'
// where 'year', 'month' and 'day' are directories and the file with name is inside. 
#[get("/")]
fn blog_post() -> Result<Template, Status> {
    Ok(Template)
}

//figure out a way to set or update a blogpost's date or something (ie moving into other directory)

//look into creating a post via uploading an html file or just a piece of text, 
//might have to create a parser that generates an html file from the text sent in a request as 
//"title, paragraph, image, paragraph... etc..." thought that seems like it might take too much time 


#[get("/posts/<name>")]
fn get_blog_posts(name:&str) -> Result<Template, Status> {
    //instead of this posts could do a regressive search trough all directories for a file with
    //such title
    let s_name : String = name.to_string(); 
    let template : Template = Template::render(s_name, context! {
        title: format!("Post {}", name)
    });
    if validate(name){
        return Ok(template);
    }
    else {
        return Err(Status::NotFound);
    }    
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, json, test, get_blog_posts])
        .attach(Template::fairing())
}
