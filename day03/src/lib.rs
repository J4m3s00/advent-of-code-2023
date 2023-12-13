use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum Field {
    Number(u32),
    Symbol(char),
    Empty,
}

#[derive(Debug)]
pub struct Board {
    width: usize,
    height: usize,
    content: Box<[Field]>,
}

impl Board {
    pub fn get_at(&self, x: i32, y: i32) -> Option<&Field> {
        if x < 0 || y < 0 {
            return None;
        }
        if x >= self.width as i32 || y >= self.height as i32 {
            return None;
        }
        let index = x as usize + (y as usize * self.width);
        match self.content.get(index) {
            Some(val) => Some(val),
            None => {
                println!("Could not get val at {x}, {y} with index {index}");
                None
            }
        }
    }

    pub fn get_num_at(&self, x: i32, y: i32) -> Option<u32> {
        if let Field::Number(num) = self.get_at(x, y).unwrap_or(&Field::Empty) {
            Some(*num)
        } else {
            None
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}

impl FromStr for Board {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut width = 0;
        let mut height = 0;
        let content = s
            .lines()
            .map(|line| {
                height += 1;
                width = line.len();
                let mut chars = line.chars(); //.collect::<Vec<_>>();
                let mut res = Vec::with_capacity(width);
                while let Some(c) = chars.next() {
                    if c.is_digit(10) {
                        let mut digit_str = String::new();
                        digit_str.push(c);
                        let mut pushed = false;
                        while let Some(d) = chars.next() {
                            if !d.is_digit(10) {
                                pushed = true;
                                res.extend(
                                    (0..digit_str.len())
                                        .map(|_| Field::Number(digit_str.parse::<u32>().unwrap())),
                                );
                                if d == '.' {
                                    res.push(Field::Empty);
                                } else {
                                    res.push(Field::Symbol(d));
                                }
                                break;
                            }
                            digit_str.push(d);
                        }
                        if !pushed {
                            res.extend(
                                (0..digit_str.len())
                                    .map(|_| Field::Number(digit_str.parse::<u32>().unwrap())),
                            );
                        }
                    } else if c == '.' {
                        res.push(Field::Empty);
                    } else {
                        res.push(Field::Symbol(c));
                    }
                }
                if res.len() != width {
                    println!("This is really wrong! {line}, {}", res.len());
                }
                res
            })
            .flatten()
            .collect::<Box<_>>();

        Ok(Self {
            content,
            width,
            height,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::Board;

    #[test]
    fn parse_board() {
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
        assert_eq!(board.width(), 10);
        assert_eq!(board.height(), 10);
    }
}
