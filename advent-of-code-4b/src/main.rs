use std::fs;

struct Bound {
    lower: u64,
    upper: u64,
}

fn bounds_from_range(range: &str) -> Bound {
    let mut nums = range.split('-');
    let lower_bound = nums
        .next()
        .expect(&format!("Invalid line in input.txt: {}", range))
        .parse::<u64>()
        .expect(&format!("Invalid line in input.txt: {}", range));
    let upper_bound = nums
        .next()
        .expect(&format!("invalid line in input.txt: {}", range))
        .parse::<u64>()
        .expect(&format!("Invalid line in input.txt: {}", range));
    return Bound {
        lower: lower_bound,
        upper: upper_bound,
    };
}
fn main() {
    let pairs = fs::read_to_string("input.txt").expect("Error reading in input.txt");
    let pairs = pairs.split('\n');

    let mut overlapping_pairs: u64 = 0;
    for pair in pairs {
        if pair.is_empty() {
            continue;
        }
        let mut ranges = pair.split(',');
        let first_bound = bounds_from_range(
            ranges
                .next()
                .expect(&format!("Invalid line in input.txt: {}", pair)),
        );
        let second_bound = bounds_from_range(
            ranges
                .next()
                .expect(&format!("Invalid line in input.txt: {}", pair)),
        );
        if ((second_bound.lower <= first_bound.upper) && (second_bound.upper >= first_bound.lower))
            || ((second_bound.upper >= first_bound.lower)
                && (second_bound.lower <= first_bound.upper))
        {
            overlapping_pairs += 1;
        }
    }
    println!("Overlapping pairs: {}", overlapping_pairs);
}
