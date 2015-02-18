#[allow(dead_code)]

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
