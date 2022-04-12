use std::fs::{File, OpenOptions, remove_file};
use std::io::{Read,Write};
#[allow(unused_variables)]
#[allow(unused_assignments)]

fn main() {
    // let mut file = File::create("src/test.csv").expect("create failed bruh");
    // file.write_all("Hello Worlds! \n".as_bytes()).expect("write failed bruh!");

    // let mut file = OpenOptions::new()
    // .append(true).open("src/test.csv")
    // .expect("cannot find or cannot open file");
    // let mut i = 0;
    // loop{
    //     file.write_all("New data! \n".as_bytes()).expect("write failed bruh!");
    //     i += 1;
    //     if i > 10{
    //         break
    //     }
    // };
    //OPEN
    let mut file = File::open("src/main.rs").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);
    //DELETE
    remove_file("src/test.csv").expect("delete failed");
    // CREATE
    let mut file_new = File::create("src/test.csv").expect("create failed bruh");
    file_new.write_all(&mut contents.as_bytes()).expect("write failed bruh!");
    
}