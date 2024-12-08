use csv::ReaderBuilder;
use std::collections::HashMap;

fn main() {
    let file_name = "../input.csv";
    let mut column_1: Vec<i64> = Vec::new();
    let mut column_2: Vec<i64> = Vec::new();

    // Specify the Reader Builder
    // mut = mutable, since we'll be configuring it with the function chain
    let mut builder = ReaderBuilder::new();
    builder.has_headers(false);

    // Read the file, unwrap the results into a Reader struct
    let result = builder.from_path(file_name);
    let mut my_reader = result.unwrap();

    // For each record, unwrap Result -> StringRecord
    // Get the components by index, unwrap, then add to
    for record in my_reader.records() {
        let line = record.unwrap();

        let value_1 = line.get(0).unwrap().parse::<i64>().unwrap();
        column_1.push(value_1);

        let value_2 = line.get(1).unwrap().parse::<i64>().unwrap();
        column_2.push(value_2);
    }

    // Sort the columns
    column_1.sort();
    column_2.sort();

    // Part 1 - Distance
    let mut distance = 0;
    let data_length = column_1.len();

    for i in 0..data_length {
        distance = distance + (column_1[i] - column_2[i]).abs();
    }

    println!("distance: {}", distance);

    // Part 2 - Similarity Score
    let mut similarity_map = HashMap::new();
    for i in 0..data_length {
        let value = column_2[i];
        similarity_map
            .entry(value)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }

    let mut similarity_score = 0;
    for i in 0..data_length {
        let value = column_1[i];
        let count = similarity_map.get(&value);

        if !count.is_none() {
            similarity_score = similarity_score + value * count.unwrap();
        }
    }

    println!("similarity score: {}", similarity_score);
}
