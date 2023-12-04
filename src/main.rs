use std::{collections::HashSet, fs};

struct Card {
    winning_numbers: Vec<u32>,
    numbers: Vec<u32>,
}

fn get_card_value(won_nums: u32) -> u32 {
    let base: i32 = 2;
    (base.pow(won_nums) / base) as u32
}

impl Card {
    fn from_string(str: &str) -> Card {
        let data_table = str.split(':').nth(1).into_iter().collect::<String>();
        let parts = data_table.trim().split('|').collect::<Vec<&str>>();

        let winning_numbers = parts[0]
            .split_whitespace()
            .map(|s| s.trim().parse::<u32>().unwrap())
            .collect();
        let numbers = parts[1]
            .split_whitespace()
            .map(|s| s.trim().parse::<u32>().unwrap())
            .collect();

        Card {
            winning_numbers,
            numbers,
        }
    }

    fn count_won_numbers(&self) -> usize {
        let winning_set: HashSet<u32> = self.winning_numbers.iter().cloned().collect();
        let number_set: HashSet<u32> = self.numbers.iter().cloned().collect();

        winning_set.intersection(&number_set).count()
    }

    fn get_value(&self) -> u32 {
        let num_won_numbers = self.count_won_numbers();
        get_card_value(num_won_numbers as u32)
    }
}

/// The `get_input` function reads the contents of a file at the given path and returns them as a vector of strings, with
/// leading and trailing whitespace removed from each line.
///
/// Arguments:
///
/// * `path`: The `path` parameter in the `get_input` function is a `String` that represents the file path from which you
/// want to read the input.
///
/// Returns:
///
/// The function `get_input` returns a `Vec<String>`, which is a vector of strings.
fn get_input(path: &str) -> Vec<String> {
    let content = fs::read_to_string(path).expect("Couldn't read input");
    let lines = content
        .lines()
        .map(|s| -> String { s.trim().to_string() })
        .collect();

    lines
}

fn main() {
    let path = "res/data.txt";
    // let path = "res/data_light.txt";
    let lines = get_input(path);
    // let mut sum = 0;
    let cards: Vec<Card> = lines.iter().map(|line| Card::from_string(line)).collect();
    let mut multiplicity = vec![1; cards.len()];

    for (idx, card) in cards.iter().enumerate() {
        let won_numbers = card.count_won_numbers();

        for _ in 0..multiplicity[idx] {
            ((idx + 1)..(idx + 1 + won_numbers)).for_each(|i| {
                if i != lines.len() {
                    multiplicity[i] += 1;
                }
            });
        }
    }
    //let sum = a.iter().fold(0, |acc, x| acc + x);
    println!("{}", multiplicity.iter().sum::<i32>())
    // println!("Sum: {}", sum);
}
