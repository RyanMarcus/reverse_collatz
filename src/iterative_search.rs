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
