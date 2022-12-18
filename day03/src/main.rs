use std::collections::HashSet;

type Bag = (HashSet<char>, HashSet<char>);

fn priority(item: char) -> u64 {
    if item.is_uppercase() {
        item as u64 - 38
    } else {
        item as u64 - 96
    }
}

fn solve_part1(bags: Vec<Bag>) {
    let total: u64 = bags
        .into_iter()
        .map(|(left, right)| {
            left.intersection(&right)
                .copied()
                .map(priority)
                .sum::<u64>()
        })
        .sum();
    println!("{total}");
}

fn solve_part2(bags: Vec<Bag>) {
    let total: u64 = bags
        .chunks(3)
        .map(|group| {
            let first = group[0].to_owned();
            let first: HashSet<char> = first.0.into_iter().chain(first.1).collect();

            let second = group[1].to_owned();
            let second: HashSet<char> = second.0.into_iter().chain(second.1).collect();

            let third = group[2].to_owned();
            let third: HashSet<char> = third.0.into_iter().chain(third.1).collect();

            let badge = first
                .intersection(&second)
                .copied()
                .collect::<HashSet<_>>()
                .intersection(&third)
                .copied()
                .next()
                .unwrap();
            priority(badge)
        })
        .sum();
    println!("{total}");
}

fn main() {
    let input = include_str!("../input.txt");

    let bags: Vec<Bag> = input
        .lines()
        .map(|line| {
            let (left, right) = line.split_at(line.len() / 2);
            let left = left.chars().collect::<HashSet<char>>();
            let right = right.chars().collect::<HashSet<char>>();
            (left, right)
        })
        .collect();
    solve_part1(bags.clone());
    solve_part2(bags);
}
