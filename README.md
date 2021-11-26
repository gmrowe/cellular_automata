# game_of_life

This is a little Game of Life kata implemented in Rust inspired by the
requirements listed here: https://codingdojo.org/kata/GameOfLife/

### Executing

The application expects the starting configuration to be passed through
standard input (sdtin). If cargo is installed on your system then the program
can be invoked by cloning the repo and entering:
```
cargo run < resources/sample_input
```
from the root directory of the project.

Currently the number of generations is hardcoded in the main() function of
main.rs. 