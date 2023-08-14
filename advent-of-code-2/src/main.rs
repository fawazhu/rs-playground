use std::fs;

fn main() {
    // Rock paper scissors tournament.
    // First column describes what the opponent will play.
    // A = rock, B = paper, C = scissors
    // X = rock, Y = paper, Z = scissors
    // Second column describes what you should play.
    // 1 point for Rock, 2 points for Paper, 3 points for Scissors added to
    // 0 point for Losing, 3 points for Draw, 6 points for Win.
    // The second column actually describes what outcome you should aim for.
    // X for lose, Y for draw, Z for win

    let strategy = fs::read_to_string("input.txt").expect("Error: input.txt does not exist.");
    let lines = strategy.split('\n');

    let mut points: u64 = 0;
    for line in lines {
        if line.len() < 3 {
            continue;
        }
        let mut line_points: u64 = 0;
        let encrypted_opponent_move = line.as_bytes()[0] as char;
        let encrypted_outcome = line.as_bytes()[2] as char;

        line_points += match encrypted_outcome {
            'X' => 0,
            'Y' => 3,
            'Z' => 6,
            _ => panic!("Input is malformed."),
        };

        if (encrypted_opponent_move == 'A' && encrypted_outcome == 'X')
            || (encrypted_opponent_move == 'B' && encrypted_outcome == 'Z')
            || (encrypted_opponent_move == 'C' && encrypted_outcome == 'Y')
        {
            line_points += 3;
        } else if (encrypted_opponent_move == 'A' && encrypted_outcome == 'Z')
            || (encrypted_opponent_move == 'B' && encrypted_outcome == 'Y')
            || (encrypted_opponent_move == 'C' && encrypted_outcome == 'X')
        {
            line_points += 2;
        } else {
            line_points += 1;
        }
        println!("{} {}", line, line_points);
        points += line_points;
    }

    let points_string = points.to_string();
    println!("Total points: {}", points_string);
}
