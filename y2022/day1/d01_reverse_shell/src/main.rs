use std::{
    io::{self, BufRead, Read},
    str::FromStr,
    string::ParseError,
};

struct Elves {
    calories: Vec<u32>,
}

impl Elves {
    pub fn get_fat(&self) -> &u32 {
        &self.calories[0]
    }

    pub fn get_fat_sum(&self, top: usize) -> u32 {
        let mut sum = 0;
        for i in 0..top {
            sum += self.calories[i];
        }
        sum
    }

    pub fn shell_sort<T: Ord + Copy>(values: &mut [T]) {
        fn insertion<T: Ord + Copy>(values: &mut [T], start: usize, gap: usize) {
            for i in ((start + gap)..values.len()).step_by(gap) {
                let val_current = values[i];
                let mut pos = i;
                while pos >= gap && values[pos - gap] > val_current {
                    values[pos] = values[pos - gap];
                    pos -= gap;
                }
                values[pos] = val_current;
            }
        }

        let mut count_sublist = values.len() / 2;
        while count_sublist > 0 {
            for pos_start in 0..count_sublist {
                insertion(values, pos_start, count_sublist);
            }
            count_sublist /= 2;
        }
    }
}

impl FromStr for Elves {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let calories = s
            .trim()
            .split("\n\n")
            .map(|elf| {
                elf.split("\n")
                    .fold(0, |acc, itm| acc + itm.parse::<u32>().unwrap())
            })
            .collect::<Vec<u32>>();
        let mut elves = Elves { calories };
        elves.calories.sort_unstable();
        Elves::shell_sort(&mut elves.calories);
        Ok(elves)
    }
}

fn main() {
    let file = include_str!("../../data.txt");
    let elves = Elves::from_str(file).expect("Failed to parse file");
    // let calories = elves.get_fat();
    // println!("Elf carrying the most calories: {}", calories);
    // dbg!(calories);
    let sum = elves.get_fat_sum(3);
    // println!("Sum of top 3 elves' calories: {}", sum);
    dbg!(sum);
    // }
}
