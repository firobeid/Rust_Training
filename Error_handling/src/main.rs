use std::fs::File;
use std::io;
use std::io::Read;

fn main() {
    let f = File::open("1.pdf");
    match f{
        Ok(f) => {
            println!("file found {:?}", f)
        }
        Err(e) => {
            println!("file not found \n{:?}", e)
        }
    }
    println!("Continue...");

    divide(Some(1));
    divide(Some(10));
    divide(None);
    // divide(Some(0));

    let a = read_user();
    println!("{:?}", a);
}

const ANSWER: i32 = 42;
fn divide(x: Option<i32>){
    match x {
        Some(0) => panic!("Cannot div by 0"),
        Some(x) => println!("result is {}", ANSWER / x),
        None => println!("None recieved, the answer stays {}", ANSWER)
    }
}

fn read_user() -> Result<String, io::Error> {
    let mut f = File::open("src/data.txt")?;
    // Ok(f)
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}