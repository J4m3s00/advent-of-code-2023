fn calculate_calibration_value(input: &str) -> usize {
    input.lines().fold(0, |res, line| {
        let start = line.chars().find(|c| c.is_digit(10));
        let end = line.chars().rev().find(|c| c.is_digit(10));

        res + format!("{}{}", start.unwrap_or('0'), end.unwrap_or('0'))
            .parse::<usize>()
            .unwrap_or(0)
    })
}

fn main() {
    let input = include_str!("cali_input.txt");
    println!("Calibration {}", calculate_calibration_value(input));
}
