pub fn parts() {
    let nums = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut p1 = 0;
    let mut p2 = 0;
    for line in include_str!("../../input/day_01.txt").split("\n") {
        let mut p1_digits: Vec<_> = vec![];
        let mut p2_digits: Vec<_> = vec![];
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
    println!("{}", p1);
    println!("{}", p2);
}
