use day03::{Board, Field};

pub fn calc(board: &Board) -> u32 {
    let mut result = 0;
    for y in 0..board.height() as i32 {
        for x in 0..board.width() as i32 {
            //println!("Get at {x} {y}");
            let cur = board.get_at(x, y);
            if let None = cur {
                println!("Not found at {x}, {y}");
            }
            if let Field::Symbol(_) = cur.unwrap() {
                let top = board.get_num_at(x, y - 1).unwrap_or(
                    board.get_num_at(x - 1, y - 1).unwrap_or(0)
                        + board.get_num_at(x + 1, y - 1).unwrap_or(0),
                );
                let bottom = board.get_num_at(x, y + 1).unwrap_or(
                    board.get_num_at(x - 1, y + 1).unwrap_or(0)
                        + board.get_num_at(x + 1, y + 1).unwrap_or(0),
                );
                let left = board.get_num_at(x - 1, y).unwrap_or(0);
                let right = board.get_num_at(x + 1, y).unwrap_or(0);
                result += top + left + right + bottom;
            }
        }
    }
    result
}

fn main() {
    let input = include_str!("input.txt");
    let res = input.parse::<Board>().unwrap();
    println!("res {:?}, {}", res.width(), res.height());
    println!("Part 1: {}", calc(&res));
}
