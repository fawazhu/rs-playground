use std::collections::HashMap;
use std::fs;

enum Uniqueness {
    Unique,
    Duplicate,
}
fn main() {
    let rucksacks = fs::read_to_string("input.txt").expect("Error reading input.txt");
    let rucksacks = rucksacks.split('\n');

    let mut rucksacks_sum: u64 = 0;
    for rucksack in rucksacks {
        let mut rucksack_iterator = rucksack.chars();
        let mut item_status: HashMap<char, Uniqueness> = HashMap::new();
        for _ in 0..(rucksack.len()/2) {
            let item = rucksack_iterator.next().unwrap();
            item_status.insert(item, Uniqueness::Unique);
        }
        for _ in (rucksack.len()/2)..rucksack.len() {
            let item = rucksack_iterator.next().unwrap();
            match item_status.get_mut(&item) {
                Some(status) => {
                    *status = Uniqueness::Duplicate;
                }
                None => {}
            }
        }
        let mut rucksack_sum: u64 = 0;
        for (key, value) in item_status.iter() {
           match *value {
               Uniqueness::Duplicate => {
                   if key.is_lowercase() {
                       rucksack_sum += (*key as u64) - ('a' as u64) + 1;
                   } else if key.is_uppercase() {
                       rucksack_sum += (*key as u64) - ('A' as u64) + 27;
                   } else {
                       panic!("Input is in an invalid format")
                   }
               }
               Uniqueness::Unique => {}
           }
        }
        rucksacks_sum += rucksack_sum;
    }
    println!("Priority sum {}", rucksacks_sum);
}
