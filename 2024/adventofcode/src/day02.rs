use std::fs::File;
use std::io::{BufRead, BufReader};

const FILE_NAME: &str = "../inputs/Day02.csv";

pub fn solution() {
    println!("\nDay 2");

    // - Open the file
    // - For each line:
    //     - Split the lne at the space
    //     - Convert each word into a string, stuff it in a vector
    //     - Check if the vector is constantly increasing / decreasing
    //     - Check if the values in the vector are all off by 1 - 3

    let mut num_safe_lines = 0;
    let reader = BufReader::new(File::open(FILE_NAME).expect("Cannot open file"));

    for line in reader.lines() {
        let mut line_vector: Vec<i32> = Vec::new();

        for string_int in line.unwrap().split_whitespace() {
            let int = string_int.parse::<i32>().unwrap();
            line_vector.push(int);
        }

        let mut diff_vector: Vec<i32> = Vec::new();
        for window in line_vector.windows(2) {
            let diff = window[1] - window[0];
            diff_vector.push(diff);
        }

        let all_positive = diff_vector.clone().into_iter().all(|x| x.is_positive());
        let all_negative = diff_vector.clone().into_iter().all(|x| !x.is_positive());
        let all_safe = diff_vector
            .clone()
            .into_iter()
            .all(|x| ((x.abs() >= 1) && (x.abs() <= 3)));

        if (all_positive || all_negative) && all_safe {
            num_safe_lines = num_safe_lines + 1;
        }
    }

    println!("  safe reports: {}", num_safe_lines);
}
