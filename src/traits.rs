

struct React{
    name:String, 
    version:String
}

trait Contains {
    fn contains(&self)-> String;
}

impl Contains for React {
    fn contains(&self) -> String {
        format!("the name of the framework is {} and the version is {}", self.name, self.version)
    }
}


pub fn maintrait(){
    let react = React{
        name: String::from("React"),
        version: String::from("18.0")
    };

    println!("the trait contains the value {:?} ",  taketimplasparameter(react));
}

// the generic and trait bound in the function says that the function can take any type of parameter as long as it implements the trait Contains and the function will return the value of the trait function contains() which is implemented for the struct React in this case.
fn taketimplasparameter<T: Contains>(a : T) {
    println!("the trait contains the value {}", a.contains());
}