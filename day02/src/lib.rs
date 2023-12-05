use std::str::FromStr;

use nom::{
    bytes::complete::take_while,
    character::{complete::alpha1, streaming::digit1},
    combinator::map_res,
    sequence::separated_pair,
    IResult,
};

#[derive(Clone)]
pub struct Bag {
    pub num_red: usize,
    pub num_green: usize,
    pub num_blue: usize,
}

pub struct Game {
    /// Number/Id of the game.
    num: usize,
    moves: Vec<Bag>,
}

impl Game {
    pub fn num(&self) -> usize {
        self.num
    }

    pub fn moves(&self) -> &[Bag] {
        &self.moves
    }
}

fn sp<'a>(i: &'a str) -> IResult<&'a str, &'a str> {
    let chars = " \t\r\n";

    // nom combinators like `take_while` return a function. That function is the
    // parser,to which we can pass the input
    take_while(move |c| chars.contains(c))(i)
}

fn digit(input: &str) -> IResult<&str, usize> {
    map_res(digit1, usize::from_str)(input)
}

fn parse_bag(input: &str) -> IResult<&str, Bag> {
    let single_color = separated_pair(digit, sp, alpha1);
    let (input, colors) =
        nom::multi::separated_list0(nom::bytes::complete::tag(", "), single_color)(input)?;
    Ok((
        input,
        Bag {
            num_blue: colors
                .iter()
                .find_map(|(num, c)| if *c == "blue" { Some(*num) } else { None })
                .unwrap_or(0),
            num_green: colors
                .iter()
                .find_map(|(num, c)| if *c == "green" { Some(*num) } else { None })
                .unwrap_or(0),
            num_red: colors
                .iter()
                .find_map(|(num, c)| if *c == "red" { Some(*num) } else { None })
                .unwrap_or(0),
        },
    ))
}

impl FromStr for Game {
    type Err = nom::Err<()>;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (input, _) = nom::bytes::complete::tag("Game ")(input)?;
        let (input, game_num) = digit(input).map_err(|_| nom::Err::Failure(()))?;
        let (input, _) = nom::bytes::complete::tag(": ")(input)?;
        let (_, bags) =
            nom::multi::separated_list0(nom::bytes::complete::tag("; "), parse_bag)(input)
                .map_err(|_| nom::Err::Failure(()))?;
        Ok(Self {
            num: game_num,
            moves: bags,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{parse_bag, Game};

    #[test]
    fn parsing_bag1() {
        let input = "5 red, 3 blue";
        let (_, bag) = parse_bag(input).unwrap();
        assert_eq!(bag.num_blue, 3);
        assert_eq!(bag.num_red, 5);
        assert_eq!(bag.num_green, 0);
    }

    #[test]
    fn parsing_bag2() {
        let input = "32 red, 3 blue, 123 green";
        let (_, bag) = parse_bag(input).unwrap();
        assert_eq!(bag.num_blue, 3);
        assert_eq!(bag.num_red, 32);
        assert_eq!(bag.num_green, 123);
    }

    #[test]
    fn parsing_game() {
        let input = "Game 32: 32 red, 3 blue, 123 green; 2 red, 3 blue";
        let game: Game = input.parse().unwrap();
        assert_eq!(game.num, 32);
        assert_eq!(game.moves.len(), 2);
        assert_eq!(game.moves[0].num_blue, 3);
        assert_eq!(game.moves[0].num_green, 123);
        assert_eq!(game.moves[0].num_red, 32);
        assert_eq!(game.moves[1].num_red, 2);
        assert_eq!(game.moves[1].num_blue, 3);
        assert_eq!(game.moves[1].num_green, 0);
    }
}
