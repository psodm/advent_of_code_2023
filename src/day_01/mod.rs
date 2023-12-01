pub fn parts() {
    let nums = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut p1 = 0;
    let mut p2 = 0;
    for line in include_str!("../../input/day_01.txt").split("\n") {
        let mut p1_digits: Vec<char> = vec![];
        let mut p2_digits: Vec<char> = vec![];
        for (i, c) in line.chars().enumerate() {
            if c.is_numeric() {
                p1_digits.push(c);
                p2_digits.push(c);
            }
            for (idx, val) in nums.iter().enumerate() {
                if line[i..].starts_with(val) {
                    match char::from_digit((idx + 1).try_into().unwrap(), 10) {
                        Some(num) => p2_digits.push(num),
                        None => {}
                    }
                }
            }
        }
        p1 = p1
            + format!("{}{}", p1_digits[0], p1_digits[p1_digits.len() - 1])
                .parse::<i32>()
                .unwrap();
        p2 = p2
            + format!("{}{}", p2_digits[0], p2_digits[p2_digits.len() - 1])
                .parse::<i32>()
                .unwrap();
    }
    println!("D01-P1: {}", p1);
    println!("D01-P2: {}", p2);
}

// The following is an alternative version, with the parts split into 1 and 2 based on
// a more idiomatic rewrite by u/AugustusLego
// on the reddit megathread for day 1:
// https://www.reddit.com/r/adventofcode/comments/1883ibu/2023_day_1_solutions/kbj8l1n/?context=3
#[allow(unused)]
pub fn solution_alternate_part1() {
    let out: Vec<u32> = include_str!("../../input/day_01.txt")
        .lines()
        .filter_map(|line| {
            let mut digits: Vec<char> = vec![];
            line.chars().for_each(|c| {
                if c.is_numeric() {
                    digits.push(c);
                }
            });
            Some(
                format!("{}{}", digits.first()?, digits.last()?).parse::<u32>().unwrap()
            )
        })
        .collect();
    println!("D01-P1: {}", out.iter().sum::<u32>());
}

#[allow(unused)]
pub fn solution_alternate_part2() {
    let nums = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let out: Vec<u32> = include_str!("../../input/day_01.txt")
        .lines()
        .filter_map(|line| {
            let mut digits: Vec<char> = vec![];
            line.char_indices().for_each(|(i, c)| {
                if c.is_numeric() {
                    digits.push(c);
                };
                for (idx, val) in  nums.iter().enumerate() {
                    if line[i..].starts_with(val) {
                        match char::from_digit((idx+1).try_into().unwrap(), 10) {
                            Some(num) => digits.push(num),
                            None => ()
                        }
                    }
                };
            });
            Some(
                format!("{}{}", digits.first()?, digits.last()?).parse::<u32>().unwrap()
            )
        })
        .collect();
    println!("D01-P1: {}", out.iter().sum::<u32>());
}