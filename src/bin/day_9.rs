fn main() {
    let input = aoc_2023_rust::helpers::input("9");
    println!("answer: {}", part1(&input));
    println!("answer part 2: {}", part2(&input));
}

fn part1(input: &str) -> isize {
    input
        .lines()
        .map(|line| {
            let mut uline = line
                .split(' ')
                .filter_map(|n| n.parse::<isize>().ok())
                .collect::<Vec<isize>>();
            let mut diffs: Vec<Vec<isize>> = Vec::new();
            diffs.push(uline.clone());
            loop {
                let mut all_zero = true;
                let mut nuline = vec![];
                let mut prev = uline[0];
                uline[1..].iter().for_each(|u| {
                    let diff = u - prev;
                    prev = *u;
                    if diff != 0 {
                        all_zero = false;
                    }
                    nuline.push(diff);
                });
                uline = nuline.clone();
                diffs.push(nuline);
                if all_zero {
                    break;
                }
            }
            diffs
                .iter_mut()
                .map(|diff| diff.pop().unwrap())
                .sum::<isize>()
        })
        .sum()
}

fn part2(input: &str) -> isize {
    input
        .lines()
        .map(|line| {
            let mut uline = line
                .split(' ')
                .filter_map(|n| n.parse::<isize>().ok())
                .collect::<Vec<isize>>();
            let mut diffs: Vec<Vec<isize>> = Vec::new();
            diffs.push(uline.clone());
            loop {
                let mut all_zero = true;
                let mut nuline = vec![];
                let mut prev = uline[0];
                uline[1..].iter().for_each(|u| {
                    let diff = u - prev;
                    prev = *u;
                    if diff != 0 {
                        all_zero = false;
                    }
                    nuline.push(diff);
                });
                uline = nuline.clone();
                diffs.push(nuline);
                if all_zero {
                    break;
                }
            }
            let mut ret = 0;
            diffs.iter().rev().for_each(|diff| {
                let d = diff[0] - ret;
                ret = d;
            });
            ret
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn it_works() {
        assert_eq!(part1(&INPUT), 114);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(&INPUT), 2);
    }
}
