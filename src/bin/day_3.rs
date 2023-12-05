fn main() {
    let input = aoc_2023_rust::helpers::input("3");
    println!("answer: {}", part1(&input));
    println!("answer part 2: {}", part2(&input));
}

struct Word {
    str: String,
    yind: usize,
    xstart: usize,
    xend: usize,
}

fn part1(input: &str) -> isize {
    let matcher = regex::Regex::new(r"\d+").unwrap();

    let mut words: Vec<Word> = vec![];
    let grid = input
        .lines()
        .enumerate()
        .map(|(yind, line)| {
            matcher.find_iter(line).for_each(|capture| {
                words.push(Word {
                    str: capture.as_str().to_string(),
                    yind,
                    xstart: capture.start(),
                    xend: capture.end(),
                });
            });
            line.chars().collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>();
    words
        .iter()
        .filter(|word| {
            (0..word.str.len())
                .any(|cind| symbol_beside(&grid, word.yind as isize, (word.xstart + cind) as isize))
        })
        .map(|word| word.str.parse::<isize>().unwrap())
        .sum()
}

fn part2(input: &str) -> isize {
    let matcher = regex::Regex::new(r"\d+").unwrap();

    let mut words: Vec<Word> = vec![];
    input.lines().enumerate().for_each(|(yind, line)| {
        matcher.find_iter(line).for_each(|capture| {
            words.push(Word {
                str: capture.as_str().to_string(),
                yind,
                xstart: capture.start(),
                xend: capture.end(),
            });
        });
    });

    let mut sum = 0;
    input.lines().enumerate().for_each(|(yind, line)| {
        line.chars().enumerate().for_each(|(xind, char)| {
            if char != '*' {
                return;
            }
            let words_near = words
                .iter()
                .filter(|word| {
                    yind.abs_diff(word.yind) <= 1
                        && (word.xstart..word.xend).any(|i| i.abs_diff(xind) <= 1)
                })
                .map(|word| word.str.clone())
                .collect::<Vec<String>>();

            if words_near.len() != 2 {
                return;
            }
            sum += words_near
                .iter()
                .map(|word_str| word_str.parse::<isize>().unwrap())
                .product::<isize>();
        });
    });
    sum
}

fn symbol_beside(grid: &Vec<Vec<char>>, yind: isize, xind: isize) -> bool {
    (0..=2).any(|yd| {
        let yy = yind + yd - 1;
        (0..=2).any(|xd| {
            let xx = xind + xd - 1;
            if yy == 0 && xx == 0
                || yy < 0
                || xx < 0
                || yy >= grid.len() as isize
                || xx >= grid[0].len() as isize
            {
                return false;
            }
            let char = grid[yy as usize][xx as usize];
            if !char.is_digit(10) && char != '.' {
                return true;
            }
            false
        })
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &'static str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..a";

    #[test]
    fn it_works() {
        assert_eq!(part1(TEST_INPUT), 4361);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(TEST_INPUT), 467835);
    }
}
