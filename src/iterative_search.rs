


pub fn calculate(steps: i64) -> i64 {


    let mut i:i64 = 1;
    loop {
        if collatz(i) == steps {
            return i;
        }
        i = i + 1;
    }

    
}

fn collatz(start: i64) -> i64 {
    let mut x = 0;
    let mut now = start;

    while now != 1 {
        now = match now % 2 {
            0 => now / 2,
            _ => 3*now + 1
        };
        x = x + 1;
    }

    return x;
}
