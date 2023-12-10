fn main() {
    let input = aoc_2023_rust::helpers::input("6");
    println!("answer: {}", part1(&input));
    println!("answer part 2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    let (records, times) = parse(input);
    times
        .iter()
        .enumerate()
        .map(|(ind, time)| {
            let record = records[ind];
            (0..*time).filter(|t| (time - t) * t > record).count()
        })
        .product()
}

fn part2(input: &str) -> usize {
    let mut iter = input.lines().map(|line| {
        line.chars()
            .filter(|c| c.is_numeric())
            .collect::<String>()
            .parse::<usize>()
            .unwrap()
    });
    let time = iter.next().unwrap();
    let record = iter.next().unwrap();
    (0..time).filter(|t| (time - t) * t > record).count()
}

fn parse(input: &str) -> (Vec<usize>, Vec<usize>) {
    let mut nums = input
        .split([' ', '\n'])
        .filter_map(|i| i.parse::<usize>().ok())
        .collect::<Vec<usize>>();

    (nums.drain((nums.len() / 2)..).collect(), nums)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn it_works() {
        assert_eq!(part1(&INPUT), 288);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(&INPUT), 71503);
    }
}
