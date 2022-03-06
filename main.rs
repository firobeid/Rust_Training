#[allow(unused_variables)]
#[allow(unused_assignments)]
#[allow(unused_variables)]
#[allow(dead_code)]
use crate::Me::Name;
use std::io;
mod modules;
mod archive;

fn main() {
    //! DOCS
    //! # MAIN Function
    //! Hello World RUST!
    //! ```
    //! fn main()
    //! ```
    //! 
    //! READ USER INPUT 
    // cargo init if folder already created | cargo new otherwise
    // ructc filename.rc  
    // RUN CARGO BUILD
    // cargo doc

    
    println!("Hello, World!");

    let mut input:String = String::new();
    println!("Please pass in an input in the line below:");

    match io::stdin().read_line(&mut input){
        Ok(_) => {
            println!("Your name is {}", input);
        },
        Err(e) => {
            println!("We have an {}", e);
        }
    }
    modules::play_movie("ET");
    modules::play_audio("ET");
    archive::arch::archiving();
    generator::rand_num_gen();
    const DEFAULT: i32 = 4;
    let mut num = [DEFAULT;15];
    num[0] = 123;
    println!("{:?}", num);
    for n in num.iter(){
        println!("{}", n);

    }
    // let _primes: Vec<i32> = Vec::new();
    let mut primes = vec![2,5,7,11];
    primes.push(17);
    println!("{:?}", primes);

    let slice = &primes[0..4];
    println!("{:?}", slice);
    
    let mut colors  = ["r", "g", "b","P"];

    update(&mut colors[1..3]);
    println!("{:?}", colors);

    let emp = Emp{
        name : String::from("John Smith"),
        company : String::from("Mirosoft"),
        age : 46
    };
    // print!("{:?}", emp);
    // print!("{}", emp.name);
    print!("{}\n", emp.detailing_emp());
    print!("{}\n", Emp::static_fn());
    let me = Name(String::from("Firo"));
    println!("{:?\n}", me);
}

#[derive(Debug)]
// Similar to Python Classes
pub struct Emp {
    name : String,
    company: String,
    age : u32
}
impl Emp{
    fn detailing_emp(&self) -> String {
        format!("name: {}, company: {}, age: {} \n", &self.name, &self.company, &self.age)
    }

    fn static_fn() -> String{
        String::from("Details of an individual")
    }
}
// enum
#[derive(Debug)]
enum Me{
    Name(String),
    Age(u32),
}

//Slicer func
fn update(colors_slice: &mut [&str]){
    colors_slice[0] = "y";
}
//modules in main
mod generator{
    use rand::{Rng, thread_rng, distributions::Alphanumeric};
    pub fn rand_num_gen() {
        let mut rng = rand::thread_rng();
        let a: i64 = rng.gen();
        let rand_string: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(30)
            .collect();
        println!("{}", a);
        println!("{}" ,rng.gen_range(0,10));
        println!("{}" ,rng.gen_range(0.0,10.0));
        println!("{}", rand_string);
    }

}



