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
        let mut best_bound = bounds_from_range(
            ranges
                .next()
                .expect(&format!("Invalid line in input.txt: {}", pair)),
        );
        for range in ranges {
            let bound = bounds_from_range(range);
            if ((bound.lower >= best_bound.lower) && (bound.upper <= best_bound.upper))
                || ((bound.lower <= best_bound.lower) && (bound.upper >= best_bound.upper))
            {
                overlapping_pairs += 1;
                break;
            }
            if bound.lower < best_bound.lower {
                best_bound.lower = bound.lower;
            }
            if bound.upper > best_bound.upper {
                best_bound.upper = bound.upper
            }
        }
    }
    println!("Overlapping pairs: {}", overlapping_pairs);
}
