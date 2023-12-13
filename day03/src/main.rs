use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
struct Number {
    n: usize,
    y: usize,
    first_x: usize,
    last_x: usize,
}

#[derive(PartialEq, Eq, Hash)]
struct Point {
    y: usize,
    x: usize,
}
struct Star {
    nums: Vec<Number>,
}

fn parse(lines: &Vec<Vec<char>>, star_map: &mut HashMap<Point, Star>) -> Vec<Number> {
    let mut nums = Vec::new();
    let mut current_num;
    let mut first_x = 0;
    let mut last_x;
    let mut in_number;
    for y in 0..lines.len() {
        current_num = String::new();
        in_number = false;
        for x in 0..lines[0].len() {
            if lines[y][x].is_ascii_digit() {
                current_num.push(lines[y][x]);
                if !in_number {
                    first_x = x;
                    in_number = true;
                }
            } else if in_number {
                last_x = x - 1;
                in_number = false;
                let n = Number {
                    n: current_num.parse::<usize>().expect("couldnt parse num"),
                    y,
                    first_x,
                    last_x,
                };
                if check_valid(lines, &n, star_map) {
                    nums.push(n);
                }
                current_num = String::new();
            }
            if x == lines[0].len() - 1 && in_number {
                let n = Number {
                    n: current_num.parse::<usize>().expect("couldnt parse num"),
                    y,
                    first_x,
                    last_x: x,
                };
                if check_valid(lines, &n, star_map) {
                    nums.push(n);
                }
            }
        }
    }
    nums
}

fn check_valid(lines: &Vec<Vec<char>>, num: &Number, star_map: &mut HashMap<Point, Star>) -> bool {
    let mut valid = false;
    let x_min = num.first_x.saturating_sub(1);
    let mut x_max = num.last_x;
    if x_max < lines[0].len() - 1 {
        x_max += 1;
    }
    let y_min = num.y.saturating_sub(1);
    let mut y_max = num.y;
    if num.y < lines.len() - 1 {
        y_max += 1;
    }
    for x in x_min..=x_max {
        for y in y_min..=y_max {
            if !".0123456789".contains(lines[y][x]) {
                valid = true;
            }
            if lines[y][x] == '*' {
                if let Some(star) = star_map.get_mut(&Point { y, x }) {
                    star.nums.push(*num);
                } else {
                    star_map.insert(Point { y, x }, Star { nums: vec![*num] });
                }
            }
        }
    }
    valid
}

fn solve(lines: &Vec<Vec<char>>, star_map: &mut HashMap<Point, Star>) -> usize {
    parse(lines, star_map).iter().fold(0, |acc, e| acc + e.n)
}

fn solve2(star_map: &HashMap<Point, Star>) -> usize {
    star_map
        .iter()
        .filter(|(_k, v)| v.nums.len() == 2)
        .map(|(_k, v)| v.nums.iter().fold(1, |acc, e| acc * e.n))
        .sum()
}

fn main() {
    let input = include_str!("day03.txt");
    let lines: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();
    let mut star_map: HashMap<Point, Star> = HashMap::new();
    println!("Part 1: {}", solve(&lines, &mut star_map));
    println!("Part 2: {}", solve2(&star_map))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_example() {
        let input = include_str!("test1.txt");
        let lines: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();
        let mut star_map = HashMap::new();
        assert_eq!(4361, solve(&lines, &mut star_map));
    }
    #[test]
    fn part2_example() {
        let input = include_str!("test1.txt");
        let lines: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();
        let mut star_map = HashMap::new();
        solve(&lines, &mut star_map);
        assert_eq!(467835, solve2(&star_map));
    }
}
