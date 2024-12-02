let mut distance = 0;
let data_length = column_1.len();

for i in 0..data_length {
    let abs_distance = (column_1[i] - column_2[i]).abs();
    distance = distance + (column_1[i] - column_2[i]).abs();
}