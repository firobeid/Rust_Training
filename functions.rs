// // Higher Order Functions:
// pub fn apply(f: fn(i32) -> i32, a: i32){
//     println!("{}",f(a))
// }
// let s = |a| (a*a);
// apply(s,6);

pub fn sum_even_sq (limit:i32){
    let mut sum = 0;
    for i in 0..{
        let isq = i * i;
        if isq > limit {break;}
        else {
            if iseven(isq){
                sum += isq;
            }
        }
    }
    println!("Sum :{}", sum);
}

pub fn hof_sum_even_sq(limit: i32)-> i32{
    let sum = (0..)
    .map(|x| x * x)
    .take_while(|&x| x <= limit)
    .filter(|x| iseven(*x))
    .fold(0, |sum, x| sum + x);
    sum
}

pub fn iseven(x:i32)-> bool{
    x % 2 == 0
}


#[macro_export]
macro_rules! name{
    ($($name: expr), *) => {$(println!("Hey {}", $name);)*}
}

