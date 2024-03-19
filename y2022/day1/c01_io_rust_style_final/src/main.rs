use std::{
    fs::File,
    io::{self, Read},
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
}

impl FromStr for Elves {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut calories = s
            .trim()
            .split("\n\n")
            .map(|elf| {
                elf.split('\n')
                    .filter_map(|itm| itm.parse::<u32>().ok())
                    .sum::<u32>()
            })
            .collect::<Vec<u32>>();
        calories.sort();
        calories.reverse();
        Ok(Elves { calories })
    }
}

fn file_open() -> Result<String, io::Error> {
    let mut open_file = File::open("../data.txt")?;
    let mut file = String::new(); // Changed to mutable
    open_file.read_to_string(&mut file)?; // Changed to read into file
    Ok(file)
}

fn main() {
    let open_file = file_open(); // Changed to assign the result to open_file
    match open_file {
        Ok(file) => {
            println!("Data successfully read");
            let elves = Elves::from_str(&file).expect("Failed to parse file");
            let sum = elves.get_fat_sum(3);
            dbg!(sum);
        }
        Err(err) => eprintln!("Error: {}", err),
    }
}
