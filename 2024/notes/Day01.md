# [Day 1](https://adventofcode.com/2024/day/1)

## Part 1

Basically, we have 2 gigantic lists and we need to:
- Sort each one
- Do an `Enum.zip` on them (no idea if this concept exists in Rust)
- Find the difference between the zipped pairs
- Find the sum of those differences.

My primary focus will be on actually getting this done, and not on optimization. So if brute force doesn't overwhelm my computer, we're going to stick with brute force. I need to learn the basics of the language first before we can start finessing.

### Load the dataset into memory
- I've saved the input to [input.csv](./input.csv)
- Looks like there's a `csv` library for Rust, in order to use it we should create a new `Cargo` project
    - [tutorial](https://blog.burntsushi.net/csv/)
    - [csv Crate docs](https://docs.rs/csv/latest/csv/)
- `cargo new --bin adventofcode` -> I'll just copy this forward to each day

### Do the thing
- Iterate over the lines, parsing out the individual values and storing them each in a vector
- Sort the vectors
- "Reduce" 

```rust
let mut distance = 0;
let data_length = column_1.len();

for i in 0..data_length {
    distance = distance + (column_1[i] - column_2[i]).abs();
}
```

Although the collection syntax is _similar_ to Elixir, _unlike_ Elixir, the upper bound is _not_ inclusive!
```elixir
for i <- 0..2, do: IO.puts(i)

0
1
2
```

```rust
for i in 0..2 {
    println!("{}", i);ZV
}

0
1
```

## Part 2

This time, we can iterate over the 2nd column and store counts in a hash map:
```rust
for i in 0..data_length {
    let value = column_2[i];
    similarity_map
        .entry(value)
        .and_modify(|counter| *counter += 1)
        .or_insert(1);
}
```

Once that's done, we can iterate over the 2nd column and multiply each value by the count we stored in the map:

```rust
let mut similarity_score = 0;
for i in 0..data_length {
    let value = column_1[i];
    let count = similarity_map.get(&value);

    if !count.is_none() {
        similarity_score = similarity_score + value * count.unwrap();
    }
}
```

The types are "inferred" here so we can initialize the map without any type declarations.
