use std::fs;


pub fn readthefile(){
    // let content = fs::read_to_string("file.txt").expect("unable to read the file"); 
    let content = fs::read_to_string("file.txt");

    match content {
        Ok(data) => println!("the content of the file is {}", data),
        Err(e) => println!("error reading the file: {}", e),
    }
}   