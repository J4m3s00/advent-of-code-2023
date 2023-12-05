use day02::{Bag, Game};

const START_BAG: Bag = Bag {
    num_red: 12,
    num_green: 13,
    num_blue: 14,
};

fn main() {
    let input = include_str!("input.txt");
    let games = input
        .lines()
        .map(|line| line.parse::<Game>().unwrap())
        .filter_map(|game| {
            match game.moves().into_iter().cloned().find(|bag| {
                bag.num_blue > START_BAG.num_blue
                    || bag.num_red > START_BAG.num_red
                    || bag.num_green > START_BAG.num_green
            }) {
                Some(_) => None,
                None => Some(game.num()),
            }
        })
        .sum::<usize>();

    println!("Anser: {}", games);
}
