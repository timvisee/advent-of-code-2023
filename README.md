# Advent of Code 2023 in Rust

My [Advent of Code 2023][aoc-2023] solutions in the Rust programming language.
This repository holds a separate Rust project for each day and part.

I attempt to develop a standalone, elegant, compact and fast solution for each
problem (two each day).

Two years ago I did the same, solving everything in under a second:

- https://timvisee.com/blog/solving-aoc-2020-in-under-a-second/
- https://github.com/timvisee/advent-of-code-2021
- https://github.com/timvisee/advent-of-code-2020

## Timings

Here is how long each solution runs with my input. All solutions are measured
(non scientifically) in [`bench.rs`](./runner/src/bin/bench.rs) on an `AMD Ryzen
9 5900X (24) @ 3.7GHz` machine running Linux.

|                                                | part A                              | part B                              |
|:-----------------------------------------------|:------------------------------------|:------------------------------------|
| [day 1](https://adventofcode.com/2023/day/1)   | [` 0.016 ms`](./day01a/src/main.rs) | [` 0.028 ms`](./day01b/src/main.rs) |
| [day 2](https://adventofcode.com/2023/day/2)   | [` 0.011 ms`](./day02a/src/main.rs) | [` 0.016 ms`](./day02b/src/main.rs) |
| [day 3](https://adventofcode.com/2023/day/3)   | [` 0.019 ms`](./day03a/src/main.rs) | [` 0.018 ms`](./day03b/src/main.rs) |
| [day 4](https://adventofcode.com/2023/day/4)   | [` 0.017 ms`](./day04a/src/main.rs) | [` 0.017 ms`](./day04b/src/main.rs) |
| [day 5](https://adventofcode.com/2023/day/5)   | [` 0.015 ms`](./day05a/src/main.rs) | [` 0.034 ms`](./day05b/src/main.rs) |
| [day 6](https://adventofcode.com/2023/day/6)   | [` 0.001 ms`](./day06a/src/main.rs) | [` 0.001 ms`](./day06b/src/main.rs) |
| [day 7](https://adventofcode.com/2023/day/7)   | [` 0.065 ms`](./day07a/src/main.rs) | [` 0.066 ms`](./day07b/src/main.rs) |
| [day 8](https://adventofcode.com/2023/day/8)   | [` 0.028 ms`](./day08a/src/main.rs) | [` 0.122 ms`](./day08b/src/main.rs) |
| [day 9](https://adventofcode.com/2023/day/9)   | [` 0.042 ms`](./day09a/src/main.rs) | [` 0.042 ms`](./day09b/src/main.rs) |
| [day 10](https://adventofcode.com/2023/day/10) | [` 0.048 ms`](./day10a/src/main.rs) | [` 0.095 ms`](./day10b/src/main.rs) |
| [day 11](https://adventofcode.com/2023/day/11) | [` 0.012 ms`](./day11a/src/main.rs) | [` 0.008 ms`](./day11b/src/main.rs) |

|              | one-by-one (1 CPU core)                  | parallel                                     |
|:-------------|:-----------------------------------------|:---------------------------------------------|
| _everything_ | [` 0.77 ms`](./runner/src/bin/runner.rs) | [` 0.32 ms`](./runner/src/bin/runner-par.rs) |

## Run solutions

Each Rust project needs a `input.txt` file to run, holding the puzzle input.
Simply create this file with your input and run the project to see the solution
appear.

```bash
# Switch to day 1a, add input, and run it
cd day01a
nano input.txt
cargo +nightly run --release

# or run everything in parallel
cd ../runner
cargo +nightly run --release --bin runner-par

# or benchmark every day
cd ../runner
cargo +nightly run --release --bin bench
```

Some solutions require Rust Nightly, that's why `+nightly` is included.

Sadly I cannot include my puzzle input anymore to make each solution easily
runnable as per Advent of Code
[FAQ](https://adventofcode.com/about#faq_copying):

> If you're posting a code repository somewhere, please don't include parts of
> Advent of Code like the puzzle text or your inputs.

## Other years

- [2024](https://github.com/timvisee/advent-of-code-2024)
- [2023](https://github.com/timvisee/advent-of-code-2023) _(current)_
- [2022](https://github.com/timvisee/advent-of-code-2022)
- [2021](https://github.com/timvisee/advent-of-code-2021)
- [2020](https://github.com/timvisee/advent-of-code-2020)
- [2019](https://github.com/timvisee/advent-of-code-2019)
- [2018](https://github.com/timvisee/advent-of-code-2018)
- [2017](https://github.com/timvisee/advent-of-code-2017)

## License

This project is released under the GNU GPL-3.0 license.
Check out the [LICENSE](LICENSE) file for more information.

[aoc-2023]: https://adventofcode.com/2023
