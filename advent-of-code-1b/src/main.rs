use std::fs;
fn main() {
    // Reindeer eat stars.
    // Minimum of 50 starts by december 25th
    // Each puzzle grants one star
    // Number of calories elf is carrying
    // Each elf writes their calories in a list seperated by a blank line.
    // Goal find the elf with most calories. 
    // Elves may run out snacks so they need 2 backups.
    // Goal find the sum of top three elves with the most calories.
    let input = fs::read_to_string("input.txt")
        .expect("Input file is not found");
    let lines = input.split('\n');

    let mut best_calories: [u64; 3] = [0; 3];
    let mut running_calories: u64 = 0;
    for line in lines {
        if line.len() == 0 {
            if running_calories > best_calories[2] {
              best_calories[2] = running_calories;
            }
            if running_calories > best_calories[1] {
              best_calories[2] = best_calories[1];
              best_calories[1] = running_calories;
            }
            if running_calories > best_calories[0] {
                best_calories[1] = best_calories[0];
                best_calories[0] = running_calories;
            }
            running_calories = 0;
        } else {
            let calories: u64 = line.parse::<u64>()
                .expect(&format!("Error parsing input file, received: {}", &line));
            running_calories += calories;
        }
    }
    println!("Highest calories");
    for i in 1..=best_calories.len() {
      let best_calories_text = best_calories[i-1].to_string();
      println!("{}. {} calories", i, best_calories_text);
    }

    let mut sum_best_calories: u64 = 0;
    for calories in best_calories {
        sum_best_calories += calories;
    }
    let sum_best_calories_text = sum_best_calories.to_string();
    println!("Total of highest: {}", sum_best_calories_text);
} 
