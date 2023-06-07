use std::{path::{Path}, fs};

pub fn validate (template_path : String) -> bool {
    let p = Path::new(format!("{}" ,&template_path).as_str()).exists();
    p
}

pub fn contains (path : &Path) -> String {
    if let Ok(mut res) = fs::read_dir(path){
        return res.next().unwrap().unwrap().path().display().to_string().replace("templates/", "");
    }
    else if let Err(res) = fs::read_dir(path){
        return res.to_string();
    }
    else{
        "something wierd".to_string()
    }
}
