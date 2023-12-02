fn solve(input: &str, line_value: fn(&str) -> u32) -> u32 {
    input.lines().fold(0, |acc, line| acc + line_value(line))
}

fn line_value1(line: &str) -> u32 {
    let mut iter = line.chars().into_iter().filter(|c| c.is_numeric());
    let first = iter.next().unwrap();
    let second = match iter.last() {
        Some(char) => char,
        None => first,
    };
    10 * first.to_digit(10).unwrap() + second.to_digit(10).unwrap()
}

fn line_value2(line: &str) -> u32 {
    line_value1(
        line.to_string()
            .replace("one", "one1one")
            .replace("two", "two2two")
            .replace("three", "three3three")
            .replace("four", "four4four")
            .replace("five", "five5five")
            .replace("six", "six6six")
            .replace("seven", "seven7seven")
            .replace("eight", "eight8eight")
            .replace("nine", "nine9nine")
            .as_str(),
    )
}

fn main() {
    let input = include_str!("day01.txt");
    println!("Part 1: {}", solve(input, line_value1));
    println!("Part 2: {}", solve(input, line_value2));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_example() {
        let input = include_str!("test.txt");
        assert_eq!(solve(input, line_value1), 142);
    }
    #[test]
    fn part2_example() {
        let input = include_str!("test2.txt");
        assert_eq!(solve(input, line_value2), 281);
    }
}
