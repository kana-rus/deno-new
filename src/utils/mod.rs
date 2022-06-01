use std::{
    fs::{create_dir, File},
    io::Write,
};

/*
pub fn parse_option(option_str: String) -> Result<&'static str, String> {
    match option_str.strip_prefix("-") {
        None => 
    }
}
*/

pub fn mkdir(dir_path: String) {
    match create_dir(&dir_path) {
        Err(err) => println!("{:?}", err.kind()),
        Ok(_) => println!("debug log: done mkdir {}", dir_path),
    }
}
pub fn echo(content: &str, file_path: String) {
    match File::create(&file_path) {
        Err(err) => println!("{:?}", err.kind()),
        Ok(mut file) => { file
            .write_all(content.as_bytes())
            .unwrap_or_else(|err| println!("{:?}", err));
            
            println!("debug log: done touch {}", file_path)
        }
    }
} 
