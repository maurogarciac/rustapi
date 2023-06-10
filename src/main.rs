pub mod templates;

#[macro_use] extern crate rocket;

use std::path::Path;
use std::env;

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

// where 'year', 'month' and 'day' are directories and the file with <any>.html.hbs name is inside. 
#[get("/<year>/<month>/<day>")]
fn blog_post(year:&str, month:&str, day:&str) -> Result<Template, Status> {
    let path_format = format!("templates/blogposts/{}/{}/{}", year, month, day);
    let path : &Path = Path::new(&path_format);
    let important : String = templates::contains(path);
    println!("found path: {} \ncwd: {}", important, env::current_dir().unwrap().display());
    if templates::validate(important.clone()) {    
        //dont worky
        //the internet rocket people said i have to implement my own responder and use .render()
        let template : Template = Template::render(important, context! {
            title: "tbd"
        });
        Ok(template)
    }
    else {
        Err(Status::NotFound)
    }
}

//TODO GET ALL BLOGPOSTS UNDER A SPECIFIC DIRECTORY
//IE: /year/ (Shows all the blogposts in every month of the whole year)
//IE: /year/month/ (shows all blogposts in specific month)
//  No need to implement by day since there will NEVER be 2 blogposts in one day. I think lmao xD.

//figure out a way to set or update a blogpost's date or something (ie moving into other directory)

//look into creating a post via uploading an htmle or just a piece of text, 
//might have to create a parser that generates an html file from the text sent in a request as 
//"title, paragraph, image, paragraph... etc..." thought that seems like it might take too much time
//
//paste.rs | might be worth looking into https://rocket.rs/v0.4/guide/pastebin/


#[get("/<name>")]
fn find_post(name:&str) -> Result<Template, Status> {
    //instead of this posts could do a regressive search trough all directories for a file with
    //such title
    if templates::validate(format!("templates/{}.html.hbs", name)){
        let template : Template = Template::render(name.to_string(), context! {
        title: format!("Post {}", name)
    });
        Ok(template)
    }
    else {
        Err(Status::NotFound)
    }    
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, json, test,blog_post ,find_post])
        .attach(Template::fairing())
}
