use std::fs;

const FILE_NAME: &str = "../inputs/Day03.txt";

fn can_parse(string: &str) -> bool {
    match string.parse::<i32>() {
        Ok(_n) => true,
        Err(_e) => false,
    }
}

pub fn solution() -> Result<(), std::io::Error> {
    println!("\nDay 3");
    let mut sum = 0;

    let input_string = fs::read_to_string(FILE_NAME)?;
    let mul_splits = input_string.split("mul(");

    for mul_split in mul_splits {
        let split_by_bracket: Vec<_> = mul_split.split(")").collect();
        let maybe_mult_string: &str = split_by_bracket[0];
        let split_by_comma: Vec<_> = maybe_mult_string.split(",").collect();

        if split_by_comma.len() == 2 {
            let num_1_string = split_by_comma[0];
            let num_2_string = split_by_comma[1];

            if can_parse(num_1_string) && can_parse(num_2_string) {
                let num_1 = num_1_string.parse::<i32>().unwrap();
                let num_2 = num_2_string.parse::<i32>().unwrap();
                sum = sum + (num_1 * num_2);
            }
        }
    }

    println!("  instruction results: {}", sum);

    Ok(())
}
