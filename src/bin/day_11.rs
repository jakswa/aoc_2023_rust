use std::collections::HashSet;

struct Parsed {
    grid: HashSet<(usize, usize)>,
}

fn main() {
    let input = aoc_2023_rust::helpers::input("11");
    println!("answer: {}", sums(&input, 1));
    println!("answer part 2: {}", sums(&input, 1000000 - 1));
}

fn sums(input: &str, add: usize) -> usize {
    let parsed = parse(input, add);
    let items = parsed
        .grid
        .iter()
        .map(|i| i.clone())
        .collect::<Vec<(usize, usize)>>();
    (0..items.len())
        .map(|l| {
            let item1 = items[l];
            ((l + 1)..items.len())
                .map(|r| {
                    let item2 = items[r];
                    item2.1.abs_diff(item1.1) + item2.0.abs_diff(item1.0)
                })
                .sum::<usize>()
        })
        .sum()
}

fn parse(input: &str, add: usize) -> Parsed {
    let mut ret = HashSet::new();
    let lines = input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let len = lines[0].len();
    let mut addedx = 0;
    (0..len).for_each(|x| {
        let mut seen = false;
        let mut addedy = 0;
        (0..lines.len()).for_each(|y| {
            let c = lines[y][x];
            if c == '#' {
                seen = true;
                ret.insert((x + addedx, y + addedy));
            }
            if !((0..len).any(|xx| lines[y][xx] == '#')) {
                addedy += add
            }
        });
        if !seen {
            addedx += add
        }
    });
    Parsed { grid: ret }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn it_works() {
        assert_eq!(sums(&INPUT, 1), 374);
    }

    #[test]
    fn part2_works() {
        assert_eq!(sums(&INPUT, 100 - 1), 8410);
    }
}
