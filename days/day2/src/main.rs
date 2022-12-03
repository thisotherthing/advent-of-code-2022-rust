use std::collections::HashMap;

fn main() {
    assert_eq!(part_a(include_str!("example.txt")), 15);
    assert_eq!(part_a(include_str!("input.txt")), 13052);
    assert_eq!(part_b(include_str!("example.txt")), 12);
    assert_eq!(part_b(include_str!("input.txt")), 13693);
}

// A for Rock
// B for Paper
// C for Scissors

// X for Rock 1
// Y for Paper 2
// Z for Scissors 3

// 0 if you lost
// 3 if the round was a draw
// 6 if you won

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Pick {
    Rock,
    Paper,
    Scissors,

    Draw,
    Lose,
    Win,
}

fn part_a(input: &str) -> i64 {
    let mapping = HashMap::from([
        ("A", Pick::Rock),
        ("X", Pick::Rock),
        ("B", Pick::Paper),
        ("Y", Pick::Paper),
        ("C", Pick::Scissors),
        ("Z", Pick::Scissors),
    ]);

    let outcomes: HashMap<Pick, HashMap<Pick, i64>> = HashMap::from([
        (
            Pick::Rock,
            HashMap::from([
                (Pick::Rock, 1 + 3),
                (Pick::Paper, 2 + 6),
                (Pick::Scissors, 3 + 0),
            ]),
        ),
        (
            Pick::Paper,
            HashMap::from([
                (Pick::Rock, 1 + 0),
                (Pick::Paper, 2 + 3),
                (Pick::Scissors, 3 + 6),
            ]),
        ),
        (
            Pick::Scissors,
            HashMap::from([
                (Pick::Rock, 1 + 6),
                (Pick::Paper, 2 + 0),
                (Pick::Scissors, 3 + 3),
            ]),
        ),
    ]);

    // dbg!(outcomes);

    let mut score = 0;

    for line in input.trim().split('\n') {
        let picks: Vec<Pick> = line.trim().split(' ').map(|a| mapping[a]).collect();

        score += outcomes[&picks[0]][&picks[1]];
    }

    score
}

fn part_b(input: &str) -> i64 {
    let mapping = HashMap::from([
        ("A", Pick::Rock),
        ("X", Pick::Lose),
        ("B", Pick::Paper),
        ("Y", Pick::Draw),
        ("C", Pick::Scissors),
        ("Z", Pick::Win),
    ]);

    let outcomes: HashMap<Pick, HashMap<Pick, i64>> = HashMap::from([
        (
            Pick::Rock,
            HashMap::from([
                (Pick::Draw, 1 + 3), // Rock
                (Pick::Win, 2 + 6),  // Paper
                (Pick::Lose, 3 + 0), // Scissors
            ]),
        ),
        (
            Pick::Paper,
            HashMap::from([
                (Pick::Lose, 1 + 0), // Rock
                (Pick::Draw, 2 + 3), // Paper
                (Pick::Win, 3 + 6),  // Scissors
            ]),
        ),
        (
            Pick::Scissors,
            HashMap::from([
                (Pick::Win, 1 + 6),  // Rock
                (Pick::Lose, 2 + 0), // Paper
                (Pick::Draw, 3 + 3), // Scissors
            ]),
        ),
    ]);

    // dbg!(outcomes);

    let mut score = 0;

    for line in input.trim().split('\n') {
        let picks: Vec<Pick> = line.trim().split(' ').map(|a| mapping[a]).collect();

        score += outcomes[&picks[0]][&picks[1]];
    }

    score
}
