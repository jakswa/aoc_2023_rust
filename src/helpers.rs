pub fn input(day: &str) -> String {
    let url = format!("https://adventofcode.com/2023/day/{}/input", day);
    ureq::get(&url)
        .set(
            "Cookie",
            &std::env::var("AOC_SESSION").expect("set AOC_SESSION"),
        )
        .call()
        .unwrap()
        .into_string()
        .unwrap()
}
