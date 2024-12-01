# [Day 1](https://adventofcode.com/2024/day/1)

Basically, we have 2 gigantic lists and we need to:
- Sort each one
- Do an `Enum.zip` on them (no idea if this concept exists in Rust)
- Find the difference between the zipped pairs
- Find the sum of those differences.

My primary focus will be on actually getting this done, and not on optimization. So if brute force doesn't overwhelm my computer, we're going to stick with brute force. I need to learn the basics of the language first before we can start finessing.

## Step 1 - Load the dataset into memory
- I've saved the input to [input.csv](./input.csv)
- Looks like there's a `csv` library for Rust, in order to use it we should create a new `Cargo` project
    - [tutorial](https://blog.burntsushi.net/csv/)
    - [csv Crate docs](https://docs.rs/csv/latest/csv/)
- `cargo new --bin adventofcode` -> I'll just copy this forward to each day

