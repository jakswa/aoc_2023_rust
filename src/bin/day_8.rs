use std::collections::HashMap;

fn main() {
    let input = aoc_2023_rust::helpers::input("8");
    println!("answer: {}", part1(&input));
    println!("answer part 2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    let map = parse(input);
    let instr_line = input.lines().next().unwrap();
    let mut curr = "AAA";
    let mut steps = 0;

    loop {
        instr_line.chars().for_each(|lr| {
            steps += 1;
            let nlr = map.get(curr).unwrap();
            match lr {
                'L' => curr = &nlr[0..3],
                'R' => curr = &nlr[5..8],
                _ => panic!("o"),
            }
        });
        if curr == "ZZZ" {
            break;
        }
    }
    steps
}

fn part2(input: &str) -> usize {
    let map = parse(input);
    let instr_line = input.lines().next().unwrap();
    let mut currs = map
        .keys()
        .filter(|k| k.ends_with('A'))
        .map(|k| *k)
        .collect::<Vec<&str>>();
    let firsts = currs
        .iter()
        .map(|scurr| {
            let mut steps = 0;
            let mut curr = scurr.clone();
            loop {
                instr_line.chars().for_each(|lr| {
                    steps += 1;
                    let nlr = map.get(curr).unwrap();
                    match lr {
                        'L' => curr = &nlr[0..3],
                        'R' => curr = &nlr[5..8],
                        _ => panic!("o"),
                    }
                });
                if curr.ends_with("Z") {
                    break;
                }
            }
            steps
        })
        .collect::<Vec<usize>>();
    lcmm(&firsts[..])
}

// stole pseudocode for these from "least common multiple" stackoverflow
fn gcd(mut a: usize, mut b: usize) -> usize {
    // Euclidean algorithm
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    return a;
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn lcmm(args: &[usize]) -> usize {
    if args.len() == 2 {
        return lcm(args[0], args[1]);
    } else {
        let arg0 = args[0];
        return lcm(arg0, lcmm(&args[1..]));
    }
}
// end stolen "least common multiple" stack overflow pseudocode

fn parse(input: &str) -> HashMap<&str, &str> {
    let mut map = HashMap::new();
    input.lines().skip(2).for_each(|line| {
        let mut kv = line.split(" = (");
        let key = kv.next().unwrap();
        let vals = kv.next().unwrap();
        map.insert(key, vals);
    });
    map
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    const INPUT2: &'static str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn it_works() {
        assert_eq!(part1(&INPUT), 6);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(&INPUT2), 6);
    }
}
