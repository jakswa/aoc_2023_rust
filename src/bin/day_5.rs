fn main() {
    let input = aoc_2023_rust::helpers::input("5");
    println!("answer: {}", part1(&input));
    println!("answer part 2: {}", part2(&input));
}

fn parse(input: &str) -> (Vec<isize>, Vec<Vec<Vec<isize>>>) {
    let mut maps = vec![];
    let mut iter = input.lines();
    let in_nums = iter
        .next()
        .unwrap()
        .split(' ')
        .filter_map(|s| s.parse::<isize>().ok())
        .collect::<Vec<isize>>();

    iter.next();
    iter.next();

    let mut map: Vec<Vec<isize>> = vec![];
    loop {
        let line_res = iter.next();
        if line_res.is_none() {
            break;
        }

        let line = line_res.unwrap();
        if line == "" {
            iter.next();
            maps.push(map);
            map = vec![];
        } else {
            map.push(
                line.split(' ')
                    .filter_map(|i| i.parse::<isize>().ok())
                    .collect(),
            );
        }
    }
    maps.push(map);
    (in_nums, maps)
}

fn part1(input: &str) -> isize {
    let (nums, maps) = parse(input);
    nums.into_iter()
        .map(|in_num| compute(in_num, &maps))
        .min()
        .unwrap()
}

fn compute(in_num: isize, maps: &Vec<Vec<Vec<isize>>>) -> isize {
    let mut x: isize = in_num;
    maps.iter().for_each(|map| {
        if let Some(slot) = map
            .iter()
            .find(|range| range[1] <= x && (range[1] + range[2]) > x)
        {
            x = (x - slot[1]) + slot[0];
        }
    });
    x
}

fn part2(input: &str) -> isize {
    let (nums, maps) = parse(input);
    nums.chunks(2)
        .map(|chunk| {
            (chunk[0]..(chunk[0] + chunk[1]))
                .map(|i| compute(i, &maps))
                .min()
                .unwrap()
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &'static str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn it_works() {
        assert_eq!(part1(TEST_INPUT), 35);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(TEST_INPUT), 46);
    }
}
