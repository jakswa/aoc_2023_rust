fn main() {
    let input = aoc_2023_rust::helpers::input("4");
    println!("answer: {}", part1(&input));
    println!("answer part 2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let mut winplays = line
                .split('|')
                .map(|chunk| {
                    chunk
                        .split([' ', '|', '\n'])
                        .filter_map(|i| i.parse::<usize>().ok())
                        .collect::<Vec<usize>>()
                })
                .collect::<Vec<Vec<usize>>>();
            let plays = winplays.pop().unwrap();
            let mut wins = winplays.pop().unwrap();
            wins.retain(|w| plays.contains(w));
            if wins.is_empty() {
                return 0;
            }
            (2 as usize).pow(wins.len() as u32 - 1)
        })
        .sum()
}

fn part2(input: &str) -> usize {
    let mut cards: Vec<usize> = vec![1; input.lines().count()];
    input
        .lines()
        .enumerate()
        .map(|(ind, line)| {
            let mut winplays = line
                .split('|')
                .map(|chunk| {
                    chunk
                        .split([' ', '|', '\n'])
                        .filter_map(|i| i.parse::<usize>().ok())
                        .collect::<Vec<usize>>()
                })
                .collect::<Vec<Vec<usize>>>();
            let plays = winplays.pop().unwrap();
            let mut wins = winplays.pop().unwrap();
            wins.retain(|w| plays.contains(w));
            for nxt in 0..wins.len() {
                cards[ind + nxt + 1] += cards[ind];
            }
        })
        .count();
    cards.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &'static str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn it_works() {
        assert_eq!(part1(TEST_INPUT), 13);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(TEST_INPUT), 30);
    }
}
