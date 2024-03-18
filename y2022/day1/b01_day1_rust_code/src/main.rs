use std::{str::FromStr, string::ParseError};

struct Elves {
    calories: Vec<u32>,
}

impl Elves {
    pub fn get_fat(&self) -> &u32 {
        return self.calories.get(0).unwrap();
    }
    pub fn get_fat_sum(&self, top: usize) -> u32 {
        let mut sum = 0;
        for i in 0..top {
            let calories = self.calories.get(i).unwrap();
            sum += calories;
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
    let file = include_str!("../../data.txt").;
    Elves::from_str(file);
    let elves = Elves {
        calories: vec![file.parse()],
    };
    let calories = elves.get_fat();
    println!("Elf carry most calories: {} ", calories);
    let sum = elves.get_fat_sum(3);
    println!("Sum of top 3 elves: {}", sum);
}
