

pub fn sqrt (num: f32) -> String{
    let mut value= num / 2.0;
    let mut i = 0;
    //V1
    while i < 10{
        value = (value + (num/value)) / 2.0;
        i += 1;
    }
    return format!("SQRT of {} = {}", num, value);
}

pub fn sqrt_v2 (num: f32) -> String{
    let mut value= num / 2.0;
    let mut i = 0;
    loop{
        value = (value + (num/value)) / 2.0;
        i += 1;
        if i > 10{
            break
        }
    };

    return format!("SQRT of {} = {}", num, value);
}

