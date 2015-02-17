pub fn calculate(steps: i64) {
    let vec = Vec<i64>::new();
    vec.push(1);

    for j in range(1, steps) {
        expand(vec);
    }

    return pickSmallest(vec);
}

fn expand(current: Vec<i64>) {
    let curr_length = current.len();
    current.reserve(curr_length * 2);

    for i in range(0, curr_length) {
        let item = current[i];
        current.push(2*item);
        if item % 3 == 1 
            && ((item - 1) / 3) % 2 == 1 
            && ((item - 1) / 3) > 1 {
                current.push((item - 1) / 3);
        }
    }
}

fn pickSmallest(current: Vec<i64>) -> i64 {
    let mut m = current[0];

    for i in current.iter() {
        m = if m < i { m } else { i }
    }

    return m;
}
