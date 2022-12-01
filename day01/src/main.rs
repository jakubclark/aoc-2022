fn solve_part1(elves: Vec<u32>) {
    let max = elves.into_iter().max().unwrap();
    println!("{max}");
}

fn solve_part2(mut elves: Vec<u32>) {
    elves.sort();
    let sum = elves.pop().unwrap() + elves.pop().unwrap() + elves.pop().unwrap();
    println!("{sum}");
}

fn main() {
    let input = include_str!("../input.txt");
    let mut elves = Vec::new();
    let mut elf = 0;
    for line in input.lines() {
        if line.is_empty() {
            elves.push(elf);
            elf = 0;
        } else {
            elf += line.parse::<u32>().unwrap();
        }
    }
    solve_part1(elves.to_owned());
    solve_part2(elves)
}
