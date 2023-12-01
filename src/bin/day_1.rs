fn main() {
    let input = aoc_2023_rust::helpers::input("1");
    println!("answer: {}", part1(&input));
    println!("answer part 2: {}", part2(&input));
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let chars = line.chars().collect::<Vec<char>>();
            let first = chars
                .clone()
                .into_iter()
                .find(|c| c >= &'0' && c <= &'9')
                .unwrap();
            let last = chars
                .into_iter()
                .rev()
                .find(|c| c >= &'0' && c <= &'9')
                .unwrap();
            first.to_digit(10).unwrap() * 10 + last.to_digit(10).unwrap()
        })
        .sum()
}

const DIGS: [&'static str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];
const RDIGS: [&'static str; 9] = [
    "eno", "owt", "eerht", "ruof", "evif", "xis", "neves", "thgie", "enin",
];

fn worddig(line: &str, digs: [&str; 9]) -> (usize, usize) {
    let word = digs
        .into_iter()
        .enumerate()
        .filter(|(index, dig)| line.contains(dig))
        .map(|(index, dig)| (line.find(dig).unwrap(), index + 1))
        .min_by_key(|(index, _dig)| index.clone());
    let digit = line
        .chars()
        .into_iter()
        .enumerate()
        .find(|(index, c)| c >= &'0' && c <= &'9')
        .map(|(index, c)| (index, c.to_digit(10).unwrap() as usize));

    if let Some(wm) = word {
        if let Some(dm) = digit {
            if wm.0 < dm.0 {
                return wm;
            } else {
                return dm;
            }
        }
    }
    word.or(digit).expect("huh? should exist")
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let liner = line.chars().rev().collect::<String>();
            worddig(line, DIGS).1 * 10 + worddig(&liner, RDIGS).1
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!(part1(&str), 142);
    }

    #[test]
    fn part2_works() {
        let str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!(part2(&str), 281);
    }
}
