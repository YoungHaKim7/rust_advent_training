trait Top<T> {
    fn top_n(self, n: usize) -> Vec<T>;
}

impl<T: PartialOrd, U: Iterator<Item = T>> Top<T> for U {
    fn top_n(self, n: usize) -> Vec<T> {
        let mut top = Vec::with_capacity(n);
        for value in self {
            for i in 0..n {
                if let Some(top_value) = top.get(i) {
                    if value > *top_value {
                        top[i..].rotate_right(1);
                        top[i] = value;
                        break;
                    }
                } else {
                    top.push(value);
                    break;
                }
            }
        }
        top
    }
}

pub fn combinator(input: &str) -> u64 {
    input
        .split("\n\n")
        .map(|batch| {
            batch
                .lines()
                .map(|line| line.parse::<u64>().unwrap())
                .sum::<u64>()
        })
        .top_n(3)
        .iter()
        .sum()
}

fn main() {
    let file = include_str!("../../data.txt");
    dbg!(combinator(file));
}
