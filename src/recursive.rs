#[allow(dead_code)]

pub fn calculate(steps: i64) -> i64 {
    return rcalculate(1, steps);
}


fn rcalculate(value_at: i64, 
              steps_left: i64) -> i64 {  
    
    if steps_left == 0 {
        return value_at;
    }
    
    
    println!("{} -> {};", value_at, 2*value_at);
    let b1 = rcalculate(2*value_at, steps_left - 1);
    
    if (value_at - 1) % 3 == 0 && 
        ((value_at - 1) / 3) % 2 == 1 &&
        ((value_at - 1) / 3) > 1 {
            println!("{} -> {};", value_at, (value_at - 1) / 3);
            let b2 = rcalculate((value_at - 1) / 3, steps_left - 1); 
            return if b1 < b2 { b1 } else { b2 };
        }
    
    return b1;
}
