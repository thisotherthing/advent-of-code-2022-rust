fn main() {
    assert_eq!(part_a(include_str!("example.txt")), 0);
    // assert_eq!(part_a(include_str!("input.txt")), 0);
    // assert_eq!(part_b(include_str!("example.txt")), 0);
    // assert_eq!(part_b(include_str!("input.txt")), 0);
}

fn part_a(input: &str) -> i64 {
    let mut amount: i64;

    for line in input.trim().split('\n') {
        amount = line.parse::<i64>().unwrap();

        dbg!(amount);
    }

    0
}

// fn part_b(input: &str) -> i64 {
//     0
// }
