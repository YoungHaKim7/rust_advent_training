use std::{str::FromStr, string::ParseError};

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
}

impl FromStr for Elves {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut calories = s
            .trim()
            .split("\n\n")
            .map(|elf| {
                elf.split("\n")
                    .fold(0, |acc, itm| acc + itm.parse::<u32>().unwrap())
            })
            .collect::<Vec<u32>>();
        calories.sort();
        calories.reverse();
        Ok(Elves { calories })
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
}
