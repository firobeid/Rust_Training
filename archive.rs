pub mod arch{
    // pub fn archiving(name: &str) {
    //     println!("Hello! {}", name);
    // }
    pub fn archiving() {
        let somevalue:i32 = 123;
        println!("Hello, world ! {}", somevalue);
        println!("binary {:o}", 5);
        println!("array {:?}", [1,2,3,4,5]);
        let somevalue = 2;
        println!("{}", somevalue);
        let (a,b,c) = (43,44,"red");
        println!("{}", a);
        let float:f32 = 4.;
        println!("{}", float);
        let char1 = "\u{1F601}";
        println!("{}", char1);
        let char1: &'static str = "H";
        println!("{}", char1);
        let mut owner = format!("Hi Iam {}","Firo");
        println!("{}", owner);
        owner.push_str(" Obeid");
        println!("{}", owner);
        let owner2 = owner.replace(" Obeid", " OB");
        println!("{}", owner2);
        const URL: &str = "google.com";
        println!("{}", URL);
        println!("{}", 10 & 3);
        let name = "Firo";
        println!("{}", say_hi(name));
    }
    pub fn say_hi(name: &str)-> String{
        let greet = format!("HEY {}!", name);
        return greet;
    }
    // pub fn main() {
    //     // for i in 1..6{
    //     //   println!("Hello1!");
    //     // }
    //     let name = "Firo";
    //     println!("{}", say_hi(name));
    // }

}