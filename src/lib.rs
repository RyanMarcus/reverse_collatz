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


mod recursive;
mod triangular_expansion;
mod iterative_search;

#[cfg(test)]
mod tests {

    #[test]
    fn rec_small_numbers() {
        assert_eq!(super::recursive::calculate(6), 10);
        assert_eq!(super::recursive::calculate(45), 361);
        //assert_eq!(super::recursive::calculate(260), 18514);
    }

    #[test]
    fn tri_exp_small_numbers() {
        assert_eq!(super::triangular_expansion::calculate(6), 10);
        assert_eq!(super::triangular_expansion::calculate(45), 361);
        //assert_eq!(super::triangular_expansion::calculate(260), 18514);
    }

    #[test]
    fn iterative_search() {
        assert_eq!(super::iterative_search::calculate(6), 10);
        assert_eq!(super::iterative_search::calculate(45), 361);
        assert_eq!(super::iterative_search::calculate(260), 18514);
    }
}
