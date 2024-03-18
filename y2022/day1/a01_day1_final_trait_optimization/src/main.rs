trait Top<T> {
    fn top<const N: usize>(self) -> [T; N];
}

impl<T: Default + PartialOrd, U: Iterator<Item = T>> Top<T> for U {
    fn top<const N: usize>(self) -> [T; N] {
        let mut top = core::array::from_fn(|_| Default::default());
        for value in self {
            for i in 0..N {
                if let Some(top_value) = top.get(i) {
                    if value > *top_value {
                        top[i..].rotate_right(1);
                        top[i] = value;
                        break;
                    }
                } else {
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
        .top::<3>()
        .iter()
        .sum()
}

fn main() {
    let file = include_str!("../../data.txt");
    dbg!(combinator(file));
}
