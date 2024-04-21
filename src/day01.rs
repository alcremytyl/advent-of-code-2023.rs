pub fn part1(input: String) {
    let lines = input.split('\n');
    let mut sum: u32 = 0;

    for line in lines {
        let left = line.chars().find_map(|c| c.to_digit(10)).unwrap_or(0);
        let right = line.chars().rev().find_map(|c| c.to_digit(10)).unwrap_or(0);
        sum += left * 10 + right;
    }

    println!("{sum}");
}

// FAILED: this one doesn't work apparently
pub fn part2(input: String) {
    let lines = input.split('\n');
    let mut sum: u32 = 0;

    for line in lines {
        let l = line
            .replace("one", "1")
            .replace("two", "2")
            .replace("three", "3")
            .replace("four", "4")
            .replace("five", "5")
            .replace("six", "6")
            .replace("seven", "7")
            .replace("eight", "8")
            .replace("nine", "9");

        let left = l.chars().find_map(|c| c.to_digit(10)).unwrap_or(0);
        let right = l.chars().rev().find_map(|c| c.to_digit(10)).unwrap_or(0);
        sum += left * 10 + right;

        println!("{left}{right} <- {line}")
    }
    println!("Sum: {sum}");
}
