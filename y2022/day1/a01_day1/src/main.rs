pub fn combinator(input: &str) -> u64 {
    input
        .split("\n\n")
        .map(|batch| {
            batch
                .lines()
                .map(|line| line.parse::<u64>().unwrap())
                .sum::<u64>()
        })
        .max()
        .unwrap()
}

fn main() {
    let file = include_str!("../../data.txt");
    dbg!(combinator(file));
}
