use std::fs;
fn main() {
    // Reindeer eat stars.
    // Minimum of 50 starts by december 25th
    // Each puzzle grants one star
    // Number of calories elf is carrying
    // Each elf writes their calories in a list seperated by a blank line.
    // Goal find the elf with most calories. 
    let input = fs::read_to_string("input.txt")
        .expect("Input file is not found");
    let lines = input.split('\n');

    let mut max_calories: u64 = 0;
    let mut running_calories: u64 = 0;
    for line in lines {
        if line.len() == 0 {
            if running_calories > max_calories {
                max_calories = running_calories;
            }
            running_calories = 0;
        } else {
            let calories: u64 = line.parse::<u64>()
                .expect(&format!("Error parsing input file, received: {}", &line));
            running_calories += calories;
        }
    }
    let max_calories_text = max_calories.to_string();
    println!("Highest calories {}", max_calories_text)
} 
