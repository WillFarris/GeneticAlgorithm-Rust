# A Simple Genetic Algorithm (in Rust this time)

This is an implementation of a genetic algorithm found at http://www.ai-junkie.com/ga/intro/gat1.html. When fed an integer number, the program will teach itself how to build a simple math formula that sums to that number.

This is a re-implementation of my previous genetic algorithm, found [here](https://github.com/WillFarris/GeneticAlgorithm). The program represents a mathematical formula in an unsigned 64-bit bitstring, where every 4 bits in the string represent either a base-10 numeric digit or a mathematical operator (`+`, `-`, `*`, `/`). To parse the string, the program looks for values which alternate between number and operator, e.g. `1+3*3/5`.


<p>This program is licensed inder the MIT license, found in the LICENSE file.</p>

## Installation
Requires Rust, which can be installed at https://www.rust-lang.org/tools/install. Once the toolchain is installed, the following commands are all that are needed to run the program:

```
git clone https://github.com/WillFarris/GeneticAlgorithm-Rust.git
cd GeneticAlgorithm-Rust
cargo run
```