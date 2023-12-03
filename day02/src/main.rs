use std::cmp::max;

struct Cubes {
    red: u32,
    green: u32,
    blue: u32,
}

impl Cubes {
    fn can_contain(&self, draw: &Self) -> bool {
        draw.red <= self.red && draw.green <= self.green && draw.blue <= self.blue
    }
}

struct Game {
    id: u32,
    draws: Vec<Cubes>,
}

impl Game {
    fn line_value1(&self, bag: &Cubes) -> u32 {
        if self.draws.iter().all(|draw| bag.can_contain(&draw)) {
            return self.id;
        } else {
            return 0;
        };
    }

    fn line_value2(&self) -> u32 {
        let mut min_cubes = Cubes {
            red: 0,
            green: 0,
            blue: 0,
        };
        self.draws.iter().for_each(|draw| {
            min_cubes.red = max(draw.red, min_cubes.red);
            min_cubes.green = max(draw.green, min_cubes.green);
            min_cubes.blue = max(draw.blue, min_cubes.blue);
        });
        min_cubes.red * min_cubes.green * min_cubes.blue
    }
}

fn parse_line(line: &str) -> Game {
    let mut iter = line.split(": ");
    let id = iter
        .next()
        .unwrap()
        .replace("Game ", "")
        .parse::<u32>()
        .unwrap();

    let draws_iter = iter.next().unwrap().split("; ");
    let mut draws = Vec::new();
    draws_iter.for_each(|draw| {
        let mut cubes = Cubes {
            red: 0,
            green: 0,
            blue: 0,
        };
        draw.split(", ").for_each(|item| {
            let mut iter = item.split(" ");
            let n = iter.next().unwrap().parse::<u32>().unwrap();
            let color = iter.next().unwrap();
            match color {
                "red" => cubes.red = n,
                "green" => cubes.green = n,
                "blue" => cubes.blue = n,
                _ => unreachable!(),
            }
        });
        draws.push(cubes);
    });
    Game { id, draws }
}

fn solve1(input: &str, bag: &Cubes) -> u32 {
    input
        .lines()
        .fold(0, |acc, line| acc + parse_line(line).line_value1(bag))
}

fn solve2(input: &str) -> u32 {
    input
        .lines()
        .fold(0, |acc, line| acc + parse_line(line).line_value2())
}

fn main() {
    let input = include_str!("day02.txt");
    let bag = Cubes {
        red: 12,
        green: 13,
        blue: 14,
    };
    println!("Part 1: {}", solve1(input, &bag));
    println!("Part 2: {}", solve2(input));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_example() {
        let input = include_str!("test1.txt");
        let bag = Cubes {
            red: 12,
            green: 13,
            blue: 14,
        };
        assert_eq!(solve1(input, &bag), 8);
    }
    #[test]
    fn part2_example() {
        let input = include_str!("test1.txt");
        assert_eq!(solve2(input), 2286);
    }
}
