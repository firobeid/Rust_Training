use std::io;


fn main() {
    //! DOCS
    //! # MAIN Function
    //! Hello World RUST!
    //! ```
    //! fn main()
    //! ```
    //! 
    //! READ USER INPUT 
    
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
}

// cargo init if folder already created | cargo new otherwise
// ructc filename.rc  
// RUN CARGO BUILD
// cargo doc

#[allow(unused_variables)]
#[allow(unused_assignments)]
#[allow(unused_variables)]
// fn main() {
//     let somevalue:i32 = 123;
//     println!("Hello, world ! {}", somevalue);
//     println!("binary {:o}", 5);
//     println!("array {:?}", [1,2,3,4,5]);
//     let somevalue = 2;
//     println!("{}", somevalue);
//     let (a,b,c) = (43,44,"red");
//     println!("{}", a);
//     let float:f32 = 4.;
//     println!("{}", float);
//     let char1 = "\u{1F601}";
//     println!("{}", char1);
//     let char1: &'static str = "H";
//     println!("{}", char1);
//     let mut owner = format!("Hi Iam {}","Firo");
//     println!("{}", owner);
//     owner.push_str(" Obeid");
//     println!("{}", owner);
//     let owner2 = owner.replace(" Obeid", " OB");
//     println!("{}", owner2);
//     const URL: &str = "google.com";
//     println!("{}", URL);
//     println!("{}", 10 & 3);
//   }
// fn say_hi(name: &str)-> String{
//  let greet = format!("HEY {}!", name);
//  return greet;
// }
// fn main() {
//   // for i in 1..6{
//   //   println!("Hello1!");
//   // }
//   let name = "Firo";
//   println!("{}", say_hi(name));
// }
