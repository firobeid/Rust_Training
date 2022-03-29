pub fn sqrt (num: &f32) -> String{
    let mut value= num / 2.0;
    let mut i = 0;
    while i < 10{
        value = (value + (num/value)) / 2.0;
        i += 1;
    }

    return format!("SQRT of {} = {}", num, value);
}