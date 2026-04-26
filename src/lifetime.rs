// andgling pointer issue 

pub fn lifetime(){
    let ans;
    
    let str1 = String::from("small");
    // {   
        let str2= String::from("longest");      
        ans = longest(&str1, &str2);
    // }
    println!("lifetimes {}", ans)
}
fn longest<'a>(str1: &'a str, str2: &'a str) -> &'a str{
    if str1.len() > str2.len() {
        str1
    }else {
        str2
    }
}


// struct with lifetimes 
struct User<'a> {
    name:&'a str
}
pub fn lifetimewithstruct(){
    
    let nameing = String::from("yash");

    let user = User{
        name: &nameing
    };

    println!("from lifetime, name is {}", user.name);
}


