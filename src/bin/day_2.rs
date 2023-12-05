fn main() {
    let input = aoc_2023_rust::helpers::input("2");
    println!("answer: {}", part1(&input));
    println!("answer part 2: {}", part2(&input));
}

const RED: u8 = 12;
const GREEN: u8 = 13;
const BLUE: u8 = 14;

fn part1(input: &str) -> u32 {
    let color_matcher = regex::Regex::new(r" (\d+) (\w+)").unwrap();
    let game_matcher = regex::Regex::new(r"Game (?<game>\d+)").unwrap();

    input
        .lines()
        .filter(|line| {
            color_matcher
                .captures_iter(line)
                .map(|i| {
                    let (_, [cnt, col]) = i.extract();
                    (cnt.parse::<u8>().unwrap(), col)
                })
                .all(|(cnt, col)| match col {
                    "red" => cnt <= RED,
                    "blue" => cnt <= BLUE,
                    "green" => cnt <= GREEN,
                    _ => panic!("what color?"),
                })
        })
        .map(|line| {
            let caps = game_matcher.captures(line).unwrap();
            caps["game"].parse::<u32>().unwrap()
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    let color_matcher = regex::Regex::new(r" (\d+) (\w+)").unwrap();

    input
        .lines()
        .map(|line| {
            let cols = color_matcher
                .captures_iter(line)
                .map(|i| {
                    let (_, [cnt, col]) = i.extract();
                    (cnt.parse::<u8>().unwrap(), col)
                })
                .collect::<Vec<(u8, &str)>>();
            ["red", "blue", "green"]
                .iter()
                .map(|color| {
                    cols.iter()
                        .filter(|(_cnt, col)| col == color)
                        .max_by_key(|(cnt, _col)| cnt)
                        .unwrap()
                })
                .map(|(cnt, _col)| *cnt as u32)
                .product::<u32>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &'static str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn it_works() {
        assert_eq!(part1(TEST_INPUT), 8);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(TEST_INPUT), 2286)
    }
}
