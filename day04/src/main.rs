fn parse_line(line: &str) -> usize {
    let mut iter = line.split(": ").last().expect("no line").split(" | ");
    let winners = iter
        .next()
        .expect("no winners")
        .split(' ')
        .filter(|x| !x.is_empty())
        .map(|n| n.parse::<usize>().expect("bad parse"))
        .collect::<Vec<usize>>();
    let my_numbers = iter
        .next()
        .expect("no numbers")
        .split(' ')
        .filter(|x| !x.is_empty())
        .map(|n| n.parse::<usize>().expect("bad parse"))
        .collect::<Vec<usize>>();

    let my_winners = my_numbers.iter().filter(|n| winners.contains(n)).count();
    match my_winners {
        0 => 0,
        _ => 2usize.pow((my_winners - 1) as u32),
    }
}

fn line_winners(line: &str) -> usize {
    let mut iter = line.split(": ").last().expect("no line").split(" | ");
    let winners = iter
        .next()
        .expect("no winners")
        .split(' ')
        .filter(|x| !x.is_empty())
        .map(|n| n.parse::<usize>().expect("bad parse"))
        .collect::<Vec<usize>>();
    let my_numbers = iter
        .next()
        .expect("no numbers")
        .split(' ')
        .filter(|x| !x.is_empty())
        .map(|n| n.parse::<usize>().expect("bad parse"))
        .collect::<Vec<usize>>();

    my_numbers.iter().filter(|n| winners.contains(n)).count()
}

fn solve(input: &str) -> usize {
    input.lines().fold(0, |acc, line| acc + parse_line(line))
}

fn solve2(input: &str) -> usize {
    let mut m = std::collections::HashMap::new();
    for (i, line) in input.lines().enumerate() {
        m.insert(i + 1, line_winners(line));
    }

    let mut dp = std::collections::HashMap::new();
    for i in (1..=input.lines().count()).rev() {
        let cards = m.get(&i).expect("no card in map");
        let mut val = *cards;
        for card in i..=i + cards {
            if let Some(v) = dp.get(&card) {
                val += v;
            }
        }
        dp.insert(i, val);
    }
    dp.values().sum::<usize>() + input.lines().count()
}

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", solve(input));
    println!("Part 2: {}", solve2(input));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_example() {
        let input = include_str!("test1.txt");
        assert_eq!(13, solve(input));
    }
    #[test]
    fn part2_example() {
        let input = include_str!("test1.txt");
        assert_eq!(30, solve2(input));
    }
}
