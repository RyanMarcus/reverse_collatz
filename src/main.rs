#![allow(unstable)]


use std::os;

mod triangular_expansion;
mod recursive;
mod iterative_search;

fn main() {

    if os::args().len() < 2 {
	println!("Error: Please provide a number as argument.");
	return;
    }

    //get the first command line argument
    let ref input_i = os::args()[1];
    let opt_i = input_i.parse::<i64>();
    
    let i = match opt_i {
        Some(x) => x,
        None => { println!("You must enter a number!"); return; }
    };


    println!("reverse collatz of {}: {}", i, iterative_search::calculate(i));
}
