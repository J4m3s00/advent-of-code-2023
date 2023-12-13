use day03::{Board, Field};

fn calc(board: &Board) -> u32 {
    let mut result = 0;
    for y in 0..board.height() as i32 {
        for x in 0..board.width() as i32 {
            if let Field::Symbol('*') = board.get_at(x, y).unwrap() {
                let mut parts = Vec::new();
                if board.get_num_at(x, y - 1).is_some() {
                    parts.push(board.get_num_at(x, y - 1).unwrap());
                } else {
                    if board.get_num_at(x - 1, y - 1).is_some() {
                        parts.push(board.get_num_at(x - 1, y - 1).unwrap());
                    }
                    if board.get_num_at(x + 1, y - 1).is_some() {
                        parts.push(board.get_num_at(x + 1, y - 1).unwrap());
                    }
                }

                if board.get_num_at(x, y + 1).is_some() {
                    parts.push(board.get_num_at(x, y + 1).unwrap());
                } else {
                    if board.get_num_at(x - 1, y + 1).is_some() {
                        parts.push(board.get_num_at(x - 1, y + 1).unwrap());
                    }
                    if board.get_num_at(x + 1, y + 1).is_some() {
                        parts.push(board.get_num_at(x + 1, y + 1).unwrap());
                    }
                }

                if board.get_num_at(x - 1, y).is_some() {
                    parts.push(board.get_num_at(x - 1, y).unwrap());
                }

                if board.get_num_at(x + 1, y).is_some() {
                    parts.push(board.get_num_at(x + 1, y).unwrap());
                }

                if parts.len() == 2 {
                    result += parts[0] * parts[1];
                }
            }
        }
    }
    result
}

fn main() {
    let input = include_str!("input.txt");
    let res = calc(&input.parse().unwrap());
    println!("Part2: {res}");
}

#[cfg(test)]
mod tests {
    use day03::Board;

    #[test]
    fn example() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        let board = input.parse::<Board>().unwrap();
        assert_eq!(super::calc(&board), 467835);
    }
}
