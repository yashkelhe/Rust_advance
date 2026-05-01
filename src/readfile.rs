use std::{fmt::{Debug, Display}, fs};

pub fn readthefile(){
    // let content = fs::read_to_string("file.txt").expect("unable to read the file"); 
    let content = fs::read_to_string("file.txt");

    match content {
        Ok(data) => println!("the content of the file is {}", data),
        Err(e) => eprintln!("error reading the file: {}", e),
    }

    // result 
    let valuefromfile =  readdeletedfile();
    match valuefromfile {
        Ok(value )=> println!("here is whats inside the file {}", value),
        Err(_) => println!("there is no data in the file ")
    }

    // option 
    let value  =    findainstring(String::from("helloworld"));
    match value{
        Some(data)=> print!("the index of the character is {}", data),
        None => print!("character not found"),
        
    }


    let vec = vec![1,2, 30];     
    println!("the first value in the vec is {}", first_string(&vec));




    let gen3  = Genericstruct{
        name:1,
        age:234
    };
    // trait bound in generic struct
    
    // let gen1 = Genericstruct{
    //     name: String ::from("name"), 
    //     age: String::from("22")
    // };
    println!("the   gen3 {:?} are" , gen3);



}   

#[derive(Debug)]
struct Genericstruct <T : std::ops::Add<Output= T >>{
     name:T,
     age: T
}

pub fn first_string<T>(s : &Vec<T>) -> &T {
    return  s.into_iter().nth(0).unwrap();
}

// never panic your code always catch the code and handle it 
// custom function which return the 
pub fn readdeletedfile() -> Result<String, std::io::Error> {
    fs::read_to_string("deleted.txt")
}


pub fn findainstring(a : String) -> Option<i32>{
    

    for (index , char ) in a.chars().enumerate(){
        if char == 'a' {
            return Option::Some(index as i32);
        }
    }

    Option::None
}