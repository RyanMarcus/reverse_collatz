#[allow(dead_code)]

pub fn calculate(steps: i64) -> i64 {
    let mut vec:Vec<i64> = Vec::new();
    vec.push(1);


    if steps > 25 {
        // ensure a large chunk of space now
        // reF: http://oeis.org/A005186
        let growth_const:f64 = 1.2637626;
        let step_diff:i64 = steps - 25;
        let guessed_required:f64 = 227.0f64 * powi(growth_const, step_diff) + 10.0f64;


        vec.reserve(guessed_required as usize);
    }

    for _ in (0..steps) {
        expand(&mut vec);
    }

    return pick_smallest(vec);
}

fn powi(base:f64, exp:i64) -> f64 {
    let mut to_r = base;

    for _ in (0..exp) {
        to_r = to_r * base;
    }

    return to_r;
}

fn expand(current: &mut Vec<i64>) {
    let curr_length = current.len();

    /*for i in current.iter() {
        print!("{} ", i);
    }
    println!("");*/

    for i in (0..curr_length) {
        let item = current[i];
        current.push(2*item);
        if item % 3 == 1 
            && ((item - 1) / 3) % 2 == 1 
            && ((item - 1) / 3) > 1 {
                current.push((item - 1) / 3);
        }

        current.swap_remove(i);
    }

}

fn pick_smallest(current: Vec<i64>) -> i64 {
    let mut m = current[0];

    for i in current.iter() {
        m = if m < *i { m } else { *i }
    }

    return m;
}
