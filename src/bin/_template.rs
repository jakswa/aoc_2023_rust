fn main() {
    let input = aoc_2023_rust::helpers::input("1");
    println!("answer: {}", part1(&input));
    println!("answer part 2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    input.len()
}

fn part2(input: &str) -> usize {
    input.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "buuum";

    #[test]
    fn it_works() {
        assert_eq!(part1(&INPUT), 5);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(&INPUT), 5);
    }
}
