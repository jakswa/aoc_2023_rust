use std::collections::HashMap;

fn main() {
    let input = aoc_2023_rust::helpers::input("7");
    println!("answer: {}", part1(&input));
    println!("answer part 2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    let mut hands = parse(input);
    hands.sort_by_key(|hand| score(&hand.0));
    hands
        .iter()
        .enumerate()
        .map(|(ind, hand)| (ind + 1) * hand.1)
        .sum()
}

fn part2(input: &str) -> usize {
    let mut hands = parse(input);
    hands.sort_by_key(|hand| joker_score(&hand.0));
    hands
        .iter()
        .enumerate()
        .map(|(ind, hand)| (ind + 1) * hand.1)
        .sum()
}

fn parse(input: &str) -> Vec<(String, usize)> {
    input
        .lines()
        .map(|line| {
            let mut spl = line.split(' ');
            let hand = spl.next().unwrap().to_string();
            let bid = spl.next().unwrap().parse::<usize>().unwrap();
            (hand, bid)
        })
        .collect()
}

fn score(hand: &str) -> usize {
    let hex = hand
        .chars()
        .map(|c| match c {
            ('0'..='9') => c,
            'T' => 'A',
            'J' => 'B',
            'Q' => 'C',
            'K' => 'D',
            _ => 'E',
        })
        .collect::<String>();
    let hex_score = usize::from_str_radix(&hex, 16).unwrap();
    rank(hand) * 1_000_000 + hex_score
}

fn joker_score(hand: &str) -> usize {
    let hex = hand
        .chars()
        .map(|c| match c {
            ('2'..='9') => c,
            'T' => 'A',
            'J' => '1',
            'Q' => 'C',
            'K' => 'D',
            'A' => 'E',
            _ => panic!("huh? {c}"),
        })
        .collect::<String>();
    let hex_score = usize::from_str_radix(&hex, 16).unwrap();
    let hand_rank = joker_rank(hand);
    hand_rank * 1_000_000 + hex_score
}

fn joker_rank(hand: &str) -> usize {
    let mut cnts: HashMap<char, usize> = HashMap::new();
    hand.chars().for_each(|c| *cnts.entry(c).or_insert(0) += 1);
    let joker_cnt = *cnts.get(&'J').unwrap_or(&0);
    let cnt_vals: Vec<usize> = cnts.into_values().collect();
    if cnt_vals.len() == 1 || (cnt_vals.len() == 2 && joker_cnt > 0) {
        return 28;
    }
    if cnt_vals.len() == 2 && [1, 4].contains(&cnt_vals[0]) {
        return 27;
    }
    if cnt_vals.len() == 2 {
        return 26;
    }
    if cnt_vals.iter().any(|i| *i == 3) {
        if joker_cnt == 1 {
            return 27;
        }
        return 25;
    }
    if cnt_vals.iter().filter(|i| **i == 2).count() == 2 {
        if joker_cnt == 1 {
            return 26;
        }
        if joker_cnt == 2 {
            return 27;
        }
        return 24;
    }
    if cnt_vals.iter().filter(|i| **i == 2).count() == 1 {
        if joker_cnt > 2 {
            panic!("hm");
        }
        if joker_cnt > 0 {
            return 25;
        }
        return 23;
    }
    if joker_cnt == 1 {
        return 23;
    }
    return 22;
}

fn rank(hand: &str) -> usize {
    let mut cnts: HashMap<char, usize> = HashMap::new();
    hand.chars().for_each(|c| *cnts.entry(c).or_insert(0) += 1);
    let cnt_vals: Vec<usize> = cnts.into_values().collect();
    if cnt_vals.len() == 1 {
        return 20;
    }
    if cnt_vals.len() == 2 && (2..=3).contains(&cnt_vals[0]) {
        return 18;
    }
    if cnt_vals.len() == 2 {
        return 19;
    }
    if cnt_vals.iter().any(|i| *i == 3) {
        return 17;
    }
    if cnt_vals.iter().filter(|i| **i == 2).count() >= 2 {
        return 16;
    }
    if cnt_vals.iter().filter(|i| **i == 2).count() == 1 {
        return 15;
    }
    return 14;
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    const INPUT2: &'static str = "QJAK8 5
23456 1
6789T 2
JQKA2 3
34567 4";

    const INPUT3: &'static str = "22333 1
33444 2
QQKKK 3
KKKKJ 4
AAJ44 5";

    #[test]
    fn it_works() {
        assert_eq!(part1(&INPUT), 6440);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(&INPUT), 5905);
    }

    #[test]
    fn part2a_works() {
        assert_eq!(part2(&INPUT2), 1 + 2 * 4 + 3 * 2 + 4 * 3 + 5 * 5);
    }

    #[test]
    fn part2b_works() {
        assert_eq!(part2(&INPUT3), 1 * 1 + 2 * 2 + 3 * 3 + 4 * 5 + 5 * 4);
    }
}
