# Reverse Collatz Calculator

Built with this Rust:

    rustc 1.0.0-nightly (522d09dfe 2015-02-19) (built 2015-02-20)

And this cargo:

    cargo 0.0.1-pre-nightly (3f74d7e 2015-02-14) (built 2015-02-14)

To compile an executable, do:

    cargo build --release

To just run it, do:

    cargo run 6 # where 6 is the sequence length

To run the tests, do:

    cargo test

The default method used is a simple iterative search that starts at 1. Inclded methods that are (and tested):

  * A simple iterative search that starts at 1 (default)
  * A recursive backtracking algorithm that searches the Collatz tree (`recursive.rs`)
  * An iterative triangular expansion method that searches the Collatz tree (`triangular_expansion.rs`)




> Copyright 2015 Ryan Marcus
>  
> This file is part of reverse_collatz.
>  
> reverse_collatz is free software: you can redistribute it and/or modify
> it under the terms of the GNU General Public License as published by
> the Free Software Foundation, either version 3 of the License, or
> (at your option) any later version.
>  
> reverse_collatz is distributed in the hope that it will be useful,
> but WITHOUT ANY WARRANTY; without even the implied warranty of
> MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
> GNU General Public License for more details.
>  
> You should have received a copy of the GNU General Public License
> along with reverse_collatz.  If not, see <http://www.gnu.org/licenses/>.
