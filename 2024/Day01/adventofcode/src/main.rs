use csv::ReaderBuilder;

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

    // Find the distance
    let mut distance = 0;
    let data_length = column_1.len();

    for i in 0..data_length {
        distance = distance + (column_1[i] - column_2[i]).abs();
    }

    println!("{}", distance);
}
