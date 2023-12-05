use day02::Game;

fn main() {
    let input = include_str!("input.txt");
    let games = input
        .lines()
        .map(|line| line.parse::<Game>().unwrap())
        .map(|game| {
            let mut min_blue = 0;
            let mut min_red = 0;
            let mut min_green = 0;
            for mov in game.moves().into_iter() {
                if mov.num_blue > min_blue {
                    min_blue = mov.num_blue;
                }
                if mov.num_red > min_red {
                    min_red = mov.num_red;
                }
                if mov.num_green > min_green {
                    min_green = mov.num_green;
                }
            }
            min_blue * min_red * min_green
        })
        .sum::<usize>();

    println!("Anser: {}", games);
}
