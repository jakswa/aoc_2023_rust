fn main() {
    let input = aoc_2023_rust::helpers::input("13");
    println!("answer: {}", part1(&input));
    println!("answer part 2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    input.split("\n\n").map(|s| ind(s).unwrap()).sum()
}

fn ind(input: &str) -> Option<usize> {
    let lines = input
        .lines()
        .map(|i| i.to_string())
        .collect::<Vec<String>>();
    reflect_ind(lines)
        .map(|i| i * 100)
        .or(reflect_ind(flip(input)))
}

fn part2(input: &str) -> usize {
    input.split("\n\n").map(|s| smudge_ind(s).unwrap()).sum()
}

fn smudge_ind(input: &str) -> Option<usize> {
    let lines = input
        .lines()
        .map(|i| i.to_string())
        .collect::<Vec<String>>();
    smudge_reflect_ind(lines)
        .map(|i| i * 100)
        .or(smudge_reflect_ind(flip(input)))
}

fn flip(input: &str) -> Vec<String> {
    let grid = input
        .lines()
        .map(|i| i.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    (0..grid[0].len())
        .map(|x| (0..grid.len()).map(|y| grid[y][x]).collect::<String>())
        .collect::<Vec<String>>()
}

fn reflect_ind(lines: Vec<String>) -> Option<usize> {
    (1..lines.len()).find(|center| {
        (*center..lines.len()).all(|i| {
            if center * 2 < i + 1 {
                return true;
            }
            count_diff(&lines[i], &lines[center * 2 - i - 1]) == 0
        })
    })
}

fn smudge_reflect_ind(lines: Vec<String>) -> Option<usize> {
    (1..lines.len()).find(|center| {
        (*center..lines.len())
            .map(|i| {
                if center * 2 < i + 1 {
                    return 0;
                }
                count_diff(&lines[i], &lines[center * 2 - i - 1])
            })
            .sum::<usize>()
            == 1
    })
}

fn count_diff(l1: &str, l2: &str) -> usize {
    l1.chars()
        .enumerate()
        .filter(|(ind, c)| *c != l2.chars().skip(*ind).next().unwrap())
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test]
    fn it_works() {
        assert_eq!(part1(&INPUT), 405);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(&INPUT), 400);
    }
}
