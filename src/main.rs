use std::{ collections::HashMap};
mod  lifetime;
mod thread;
mod readfile;
mod createmultithreadcalculator;



fn main() {

     lifetime::lifetime();
     lifetime::lifetimewithstruct();
     thread::implthread();
     thread::talk_to_multiple_thread();
     createmultithreadcalculator::multithreadcalculator();
     readfile::readthefile();

     
    let mut s = Vec::new();
     s.push(1);
     s.push(2);   
     s.push(5);   
     s.push(6);   
     s.push(3);   
     s.push(10);   
     s.push(100);   
    //  debug trait to print the vector
    
    println!("{:?}", returnevenvalue(& s));
    // i know this is giving the error because the vector is moved to the function and we are trying to print it again after the funcation call 
    // so for that we can use the reference of the vector instead of moving it to the function 
    println!("{:?}", s);


    // hashMap 

    let mut map = HashMap::new();
    map.insert("key1", String::from("value1"));
    map.insert("key2", String::from("value2"));

    println!("hashmap is here {:?}", map);
    println!("hashmap is here {:?}", map.get("key1"));

    // want to print the specific then use the match 
    
    match map.get("key1") {
        Some(value) => println!("Found value: {}", value),
        None => println!("Key not found"),
    }


    // Tuple Vector 
 
    let tuple:Vec<(String, i32)> = vec![(String::from("key1"),1), (String::from("key2"),2)];
    println!("Tuple vector is here {:?}", tuple);
    
    // function to return the specific key value in hashmap 

    println!("the hashmap return this value from the tuple  {:?}", returnhashmapvalue(&tuple));

    // simple tuple 
    //  A reference &str is a "fat pointer" that stores both the address of the data and its length
    // Rust needs to know the exact size of variables at compile time to allocate space on the stack. A str (string slice) is just a sequence of UTF-8 bytes, and its length can vary
    let simple_tupl: ( &str, i32, f64, bool ) = ("234234", 1, 2.0, true);

    println!("Simple tuple is here {:?}", simple_tupl);
    println!("Simple tuple is here {:?}", simple_tupl.0);
    

    // Iterator 

    let v1 = vec![1,2,3];

    // iter is borrowing the vector and returning an iterator that yields references to the elements in the vector.
    let v1_iter = v1.iter();
    // v1_iter is not the owner of the v1 
    for i in v1_iter {
        println!("the value is here {}", i);
    }
    println!("can print the v1 {:?}", v1);


    // Mutable Iterator 

    let mut v2 = vec![12,23,34];

    let v2_mut_iter = v2.iter_mut();

    for  i in v2_mut_iter {
        *i += *i + 1; // dereference the mutable reference to modify the value
    }

    // iterator 

    let nums = vec![1,2 ,3];

    let mut nums_iterator  = nums.iter();

    // if some value then print it else breck it    
    while let Some(val) = nums_iterator.next(){
        print!("{}",val);
    } 

    
    // into_iterator 
    // into iterator takes the ownership of the vector and then if you try to print the original vector it will gets print as 
    // ownership has been moved 
    let into  = vec!(1,2,3);

    let into_iternator = into.into_iter();

    for i in into_iternator{
        print!("{}", i);
    }
    // print!("{}",into);


    // Consumed Iterator adaptors 

    let vec = vec!(1,2,4);
    let vec_i = vec.iter();
    let sum:i32 = vec_i.sum();
    // now yoveu wont able to use the vec_i again as its consumed in above line to make the sum 
    print!("the sum vec using the iterator Sum which take the self as input  {}",sum);

    // Iterator adaptors 

    let vec = vec!(1,2,3);
    let vec_iterator = vec.iter();
    // lets map it  
    let vec_iterator_1 = vec_iterator.map(|x| x + 1);

    for i in vec_iterator_1 {
        print!("{}", i);
    }



    let vecing = vec!(123,123,2,1111);
    let vecing_iterator = vecing.iter();

    let filter = vecing_iterator.filter(|x| *x % 2 ==1 ).map(|x| x * 2);
    for i in filter {
        println!("filter out the odd numbers{} " ,i);

    }


    // we have learned the how to convert the vec into the iterator 
    // now lets see how we convert the iterator into Vec /

    let vec = vec!(1,2,2);
    let vec_i = vec.iter().filter(|x| **x == 2).map(|x| x * 2);
    let print : Vec<i32>= vec_i.collect();
    println!("converting the iterator into the vector {:?}",print);



    // create tuple then convert it into hashmap then convert it into the iterator then convert vector again 

    let tuple_vec = vec![(String::from("name"), 1), (String::from("name2"), 2)];

    // directly convert to hashmap
    let newhashmap: HashMap<_, _> = tuple_vec.into_iter().collect();

    for (k, v) in &newhashmap {
       println!("{} {}", k, v);
    }

    // convert back to vector
    let hash_vector: Vec<(&String, &i32)> = newhashmap.iter().collect();

    println!("{:?}", hash_vector);


    // String And Slice 
    // and this is slice slice is basically provide you the view window for the string which has referance of
    // of that window but it does not have the owner ship 
    let stringing = String::from("yash kelhe");
    let slice = slicing(&stringing);
        print!("{}",slice);


    // what is the string literal 
    // why we are not using the string = "yash kelhe"
    // because  in when it gets build / compile that time it directly hardcode it 
    // but when you create using string::from then it stores it memory 


    let theuser = User {
        name: String::from("yash"), 
        age:23
    };

    println!("the name of the user is {}",theuser.summarize());
    println!("the name of the user is {}",theuser.nameis());
    println!("the name of the user is {}",theuser.ageis());

    // another way to implement the trait by giving the the trait as parameter which you can invoke by giving the struck 
    // which has that behavior  
    
    notify(&theuser);
    let unitstruct = Fix;
    notify(&unitstruct);


    // generic bound traits  
    // means the struct that your passing should have generic trait that bound for that fuction should be impl to struct 
    let anyuser = Userany {
        addresss: String::from("yashkelhe"),
        Ipaddress: String::from("123.123.122.0")
    };


    notifyany(anyuser);


    // understand this very important 
    fn apply<F>(f: F)
        where
            F: Fn(i32) -> i32,
        {
            println!("{}", f(5));
        }

    apply(|x| x + 1);

        

    let user11  = Newusers {
        name: String::from("yash"),
        age: 23
    };

    print!("the name of the user is {}", user11.new());
    print!("the static function is here {}", Newusers::new1());
    
}


struct Newusers {
    name:String,
    age:usize
}

impl Newusers{
    // dynamic 
    fn new(&self)-> String{
        return self.name.clone();
    }

    // static 
    fn new1()-> String{
        return String::from("this is static function");
    }
}

struct Userany {
    addresss : String,
    Ipaddress : String
}
trait Fixany {
    fn name(&self) -> String {
        format!("bound to generic trait is here ")
    }
}
trait Summarizeany {
    fn nameany(&self) -> String{
        format!("bound to generic trait is here ")
    }
}

impl Fixany for Userany{}
impl Summarizeany for Userany{} 

fn notifyany<T: Fixany + Summarizeany>(a : T) {
    println!("print function {} ", a.name());
}



pub fn notify(item : &impl System){
    println!("the first trait in function {}",item.summarize());
    println!("the first trait in function{} ", item.nameis());
    println!("the first trait in function {}", item.ageis());
}

// unit stuck 
pub struct Fix;


impl System for Fix{
     fn summarize(&self) -> String {
        String::from("I am a unit struct Fix")
    }

    fn nameis(&self) -> String {
        String::from("No name available")
    }

    fn ageis(&self) -> String {
        String::from("No age available")
    }
}

pub trait System {
     fn summarize(&self) -> String ;
     fn nameis(&self)-> String;
     fn ageis(&self)-> String;
}

struct User {
    name: String,
    age: i32
}

impl System for User {
     fn summarize(&self) -> String {
        format!("name is {}, and age is {}", self.name, self.age)
    }

    fn nameis(&self)-> String{
        format!("name can be {}", self.name)
    }
    fn ageis(&self)-> String {
        format!("if i gusse the age then it will be {}", self.age)
    }
    
}

struct Newstruct{
    name:String,
    age: u32
} 
trait ContainsImplimentation{

    fn animal()-> String{
        format!("this is the trait where trait contains the blueprint function with implementation ")
    }
}
// now you dont have to implement the function implementation here as its already done in the trait 
impl ContainsImplimentation for Newstruct{}




// fn generictypefunction<T: std::cmp::PartialOrd>(a:T, b: T) -> T{
//     if a > b { 
//         a
//     }else {
//         b
//     }
// }
fn slicing(s : &String ) -> &str{
    let mut str_slice_indx = 0;

    for (_,i) in s.chars().enumerate() {
        if i == ' '{
            break ;
        }
        str_slice_indx = str_slice_indx + 1;
    } 

    return    &s[0..str_slice_indx];
}



fn returnhashmapvalue(tuple : &Vec<(String, i32)>)-> HashMap<String,i32>{

    let mut map = HashMap::new();

    for (key, value) in tuple{
        if key == "key1"{
            map.insert(key.clone(), *value);
        }
    }
     map
}


fn returnevenvalue(s : &Vec<i32>) -> Vec<i32> {
    let mut even_values = Vec::new();
    for i in s {
        // 
        if *i % 2 == 0 {
            even_values.push(*i);
        }

        // you an do this also to dereferance it 
        //  if &i % 2 == 0 {
        //         even_values.push(*i);
        //  }

        // or 
    }
    // for i in s.iter().copied() {
    //     if i % 2 == 0 {
    //         even_values.push(i);
    //     }
    // }
    even_values 
}