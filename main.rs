#[allow(unused_variables)]
#[allow(unused_assignments)]
#[allow(unused_variables)]
#[allow(dead_code)]
use crate::Suit::{Spade, Heart};
// use crate::Me::Name;
// use std::io;
mod modules;
mod archive;
mod sqrt;

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
    
    let _ans = sqrt::sqrt(&42.0);
    println!("{}", _ans);
    
    // println!("{}", );
    // println!("Hello, World!");

    // let mut input:String = String::new();
    // println!("Please pass in an input in the line below:");

    // match io::stdin().read_line(&mut input){
    //     Ok(_) => {
    //         println!("Your name is {}", input);
    //     },
    //     Err(e) => {
    //         println!("We have an {}", e);
    //     }
    // }
    // print_choice(Heart);
    // print_choice(Spade);
    
    // country_code(32);
    // country_code(961);
    // country_code(1);
    // country_code(-100);
    // for i in -2..=15{
    //     println!("{}: Quantity is {}", i ,get_quantity(i));
    // }
    // for (pos, i) in (1..=20).enumerate() {
        
    //     let nb = pos+2;
    //     println!("{0} * {0} = {1}",nb, i*i);
    // }
    // modules::play_movie("ET");
    // modules::play_audio("ET");
    // archive::arch::archiving();
    // generator::rand_num_gen();
    // const DEFAULT: i32 = 4;
    // let mut num = [DEFAULT;15];
    // num[0] = 123;
    // println!("{:?}", num);
    // for n in num.iter(){
    //     println!("{}", n);
    // }
    // // let _primes: Vec<i32> = Vec::new();
    // let mut primes = vec![2,5,7,11];
    // primes.push(17);
    // println!("{:?}", primes);

    // let slice = &primes[0..4];
    // println!("{:?}", slice);
    
    // let mut colors  = ["r", "g", "b","P"];

    // update(&mut colors[1..3]);
    // println!("{:?}", colors);

    // let emp = Emp{
    //     name : String::from("John Smith"),
    //     company : String::from("Mirosoft"),
    //     age : 46
    // };
    // // print!("{:?}", emp);
    // // print!("{}", emp.name);
    // print!("{}\n", emp.detailing_emp());
    // print!("{}\n", Emp::static_fn());
    // let me = Name(String::from("Firo"));
    // println!("{:?\n}", me);
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
// enum: A struct can contain both data variables and methods. 
//Enum can only contain data types.
#[derive(Debug)]
enum Me{
    Name(String),
    Age(u32),
}

enum Suit {
    Heart,
    Spade,
    Club,
    Diamond
}

fn print_choice(choice: Suit) {
    match choice {
       Heart => { println!("\u{2665}\r") }
       Spade => {println!("\u{2665}")}
       Club => {println!("\u{2665}")}
       Diamond => {println!("\u{2665}")}

    }
}

fn country_code(code: i32) {
    let country = match code{
        1 => "US",
        44 => "UK",
        961 => "LB",
        _ => "Invalid"
    };
    println!("Country is {}", country);
}

fn get_quantity(amount: i32) -> &'static str{
    return match amount {
        0 => "No",
        2 | 1 => "One or two",
        3..=7 => "A few",
        _ if (amount % 2 == 0) => "Even number greater then 6",
        _ if (amount % 2 != 0) => "Odd number greater then 7",
        _ if (amount < 0) => "Negative number can be a quantity",
        _ => "Invalid"
    };
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
        let num: i32 = rng.gen_range(0,200);
        let rand_string: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(30)
            .collect();
        if num >= 100 {
            println!("{} Greater the 100", num);
        } 
        else{
            println!("{} Less the 100", num);
        }
        println!("{}", a);
        println!("{}" ,rng.gen_range(0,10));
        println!("{}" ,rng.gen_range(0.0,10.0));
        println!("{}", rand_string);
    }

}





