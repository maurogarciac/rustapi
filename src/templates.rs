use std::path::Path;

pub fn validate (template_name : &str) -> bool {
    let p = Path::new(format!("templates/{}.html.hbs" ,&template_name).as_str()).exists();
    p
}

pub fn contains (path : &Path) -> String {
    let p : &Path = path;


    //find file under the path
    //return file extension
        
    "asdfasd".to_string()
}
