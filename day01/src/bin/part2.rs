fn calcaulte_calibration_value_with_words(input: &str) -> usize {
    input.lines().fold(0, |res, line| {
        let mut nums: Vec<usize> = Vec::new();
        let mut ptr = &line[..];
        while ptr.len() != 0 {
            if ptr.starts_with("one") {
                nums.push(1)
            } else if ptr.starts_with("two") {
                nums.push(2)
            } else if ptr.starts_with("three") {
                nums.push(3)
            } else if ptr.starts_with("four") {
                nums.push(4)
            } else if ptr.starts_with("five") {
                nums.push(5)
            } else if ptr.starts_with("six") {
                nums.push(6)
            } else if ptr.starts_with("seven") {
                nums.push(7)
            } else if ptr.starts_with("eight") {
                nums.push(8)
            } else if ptr.starts_with("nine") {
                nums.push(9)
            } else if let Some(digit) = ptr.chars().next().unwrap().to_digit(10) {
                nums.push(digit as usize);
            }

            ptr = &ptr[1..]
        }

        let add = match (nums.first(), nums.last()) {
            (Some(f), Some(l)) => f * 10 + l,
            _ => panic!("Could not find num. {}", line),
        };

        res + add
    })
}

fn main() {
    let input = include_str!("cali_input.txt");
    println!("Answer {}", calcaulte_calibration_value_with_words(input));
}
