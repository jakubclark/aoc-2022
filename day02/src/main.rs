fn solve_part1(input: &str) {
    let total = input
        .lines()
        .map(|round| {
            match round {
                // Rock Rock
                "A X" => 1 + 3,
                // Rock Paper
                "A Y" => 2 + 6,
                // Rock Scissors
                "A Z" => 3 + 0,

                // Paper Rock
                "B X" => 1 + 0,
                // Paper Paper
                "B Y" => 2 + 3,
                // Paper Scissors
                "B Z" => 3 + 6,

                // Scissors Rock
                "C X" => 1 + 6,
                // Scissors Paper
                "C Y" => 2 + 0,
                // Scissors Scissors
                "C Z" => 3 + 3,
                _ => unreachable!(),
            }
        })
        .sum::<u64>();
    println!("{total}");
}

fn solve_part2(input: &str) {
    let total = input
        .lines()
        .map(|round| {
            match round {
                // Rock LOSS -> Pick Scissors
                "A X" => 0 + 3,
                // Rock DRAW -> Pick Rock
                "A Y" => 3 + 1,
                // Rock WIN -> Pick Paper
                "A Z" => 6 + 2,

                // Paper LOSS -> Pick Rock
                "B X" => 0 + 1,
                // Paper DRAW -> Pick Paper
                "B Y" => 3 + 2,
                // Paper WIN -> Pick Scissors
                "B Z" => 6 + 3,

                // Scissors LOSS -> Pick Paper
                "C X" => 0 + 2,
                // Scissors DRAW -> Pick Scissors
                "C Y" => 3 + 3,
                // Scissors WIN -> Pick Rock
                "C Z" => 6 + 1,
                _ => unreachable!(),
            }
        })
        .sum::<u64>();
    println!("{total}");
}

fn main() {
    let input = include_str!("../input.txt");
    solve_part1(input);
    solve_part2(input);
}
