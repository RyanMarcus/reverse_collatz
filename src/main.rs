#![allow(unstable)]

/*

Copyright 2015 Ryan Marcus

This file is part of reverse_collatz.

reverse_collatz is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

reverse_collatz is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with reverse_collatz.  If not, see <http://www.gnu.org/licenses/>.
*/

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


    println!("{}", iterative_search::calculate(i));
}
