# Reverse Collatz Calculator

Built with this Rust:

    rustc 1.0.0-dev

And this cargo:

    cargo 0.0.1-pre-nightly (3f74d7e 2015-02-14) (built 2015-02-14)

It has additionally been tested on this system:

    $ uname -srvm 
    Linux 3.17.3-1-ARCH #1 SMP PREEMPT Fri Nov 14 23:13:48 CET 2014 x86_64
    $ rustc --version
    rustc 1.0.0-dev
    $ cargo --version
    cargo 0.0.1-pre-nightly (918a415 2015-02-17) (built 2015-02-17)

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
