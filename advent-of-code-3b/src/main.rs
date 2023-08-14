use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

enum Uniqueness {
    Unique,
    Duplicate,
    Triplicate,
}
fn main() {
    let rucksacks = fs::read_to_string("input.txt").expect("Error reading input.txt");
    let mut rucksacks = rucksacks.split('\n');

    let mut group_sum: u64 = 0;
    'group: loop {
        let mut group_item_status: HashMap<char, Uniqueness> = HashMap::new();
        for _ in 1..=3 {
            match rucksacks.next() {
                Some(rucksack) => {
                    let mut unique_items: HashSet<char> = HashSet::new();
                    for item in rucksack.chars() {
                        unique_items.insert(item);
                    }
                    for item in unique_items {
                        match group_item_status.get_mut(&item) {
                            Some(uniqueness) => {
                                *uniqueness = match uniqueness {
                                    Uniqueness::Unique => Uniqueness::Duplicate,
                                    Uniqueness::Duplicate => Uniqueness::Triplicate,
                                    Uniqueness::Triplicate => Uniqueness::Triplicate,
                                }
                            }
                            None => {
                                group_item_status.insert(item, Uniqueness::Unique);
                            }
                        };
                    }
                }
                None => break 'group,
            }
        }
        for (key, value) in group_item_status.iter() {
            match *value {
                Uniqueness::Triplicate => {
                    if key.is_lowercase() {
                        group_sum += (*key as u64) - ('a' as u64) + 1;
                    } else if key.is_uppercase() {
                        group_sum += (*key as u64) - ('A' as u64) + 27;
                    } else {
                        panic!("Input is in an invalid format")
                    }
                }
                _ => {}
            }
        }
    }
    println!("Group sum {}", group_sum);
}
