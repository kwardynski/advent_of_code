use std::fs::File;
use std::io::{BufRead, BufReader};

const FILE_NAME: &str = "../inputs/Day02.csv";

fn parse_line(line: &String) -> Vec<i32> {
    let mut line_vector: Vec<i32> = Vec::new();

    for string_int in line.split_whitespace() {
        let int = string_int.parse::<i32>().unwrap();
        line_vector.push(int);
    }

    return line_vector;
}

fn calculate_diffs(vector: &Vec<i32>) -> Vec<i32> {
    let mut diff_vector: Vec<i32> = Vec::new();

    for window in vector.windows(2) {
        let diff = window[1] - window[0];
        diff_vector.push(diff);
    }

    return diff_vector;
}

fn is_report_safe(vector: &Vec<i32>) -> bool {
    let all_positive: bool = vector.into_iter().all(|x| x.is_positive());
    let all_negative: bool = vector.into_iter().all(|x| !x.is_positive());
    let all_steps_safe: bool = vector
        .into_iter()
        .all(|x| ((x.abs() >= 1) && (x.abs() <= 3)));

    return (all_positive || all_negative) && all_steps_safe;
}

pub fn solution() {
    println!("\nDay 2");

    // Part 1
    // - Open the file
    // - For each line:
    //     - Split the lne at the space
    //     - Convert each word into a string, stuff it in a vector
    //     - Check if the vector is constantly increasing / decreasing
    //     - Check if the values in the vector are all off by 1 - 3
    let mut num_safe_lines = 0;
    let reader = BufReader::new(File::open(FILE_NAME).expect("Cannot open file"));

    for line in reader.lines() {
        let line_vector = parse_line(&line.unwrap());
        let diff_vector = calculate_diffs(&line_vector);
        let safe = is_report_safe(&diff_vector);

        if safe {
            num_safe_lines = num_safe_lines + 1
        }
    }

    println!("  safe reports: {}", num_safe_lines);

    // Part 2
    // Same as day 1, except for unsafe lines ->
    //   - Iterate over the vector, removing 1 element at a time
    //   - Determine safety, short circuit at first safety
    let mut num_safe_lines = 0;
    let reader = BufReader::new(File::open(FILE_NAME).expect("Cannot open file"));

    for line in reader.lines() {
        let line_vector = parse_line(&line.unwrap());
        let diff_vector = calculate_diffs(&line_vector);
        let safe = is_report_safe(&diff_vector);

        if safe {
            num_safe_lines = num_safe_lines + 1;
        } else {
            let vector_length = line_vector.len();

            for i in 0..vector_length {
                let mut attempt_vector = line_vector.clone();
                attempt_vector.remove(i);

                let attempt_diff_vector = calculate_diffs(&attempt_vector);
                let attempt_safe = is_report_safe(&attempt_diff_vector);

                if attempt_safe {
                    num_safe_lines = num_safe_lines + 1;
                    break;
                }
            }
        }
    }

    println!("  safe reports with buffer: {}", num_safe_lines);
}
