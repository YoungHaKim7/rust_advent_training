#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use test::{black_box, Bencher};

    #[bench]
    fn bench_pow(b: &mut Bencher) {
        // Optionally include some setup
        fn combinator(input: &str) -> u64 {
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

        pub fn combinator_iterator(input: &str) -> u64 {
            input
                .split("\n\n")
                .map(|batch| {
                    batch
                        .lines()
                        .map(|line| line.parse::<u64>().unwrap())
                        .sum::<u64>()
                })
                .sorted()
                .rev()
                .take(3)
                .sum()
        }

        let file = include_str!("../../data.txt");
        let x = combinator(&file);
        let y = combinator_iterator(&file);

        b.iter(|| {
            // Inner closure, the actual test
            for i in 1..100 {
                black_box(x.pow(y.try_into().unwrap()).pow(x.try_into().unwrap()));
            }
        });
    }
}
