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


