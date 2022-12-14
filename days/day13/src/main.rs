fn main() {
    assert_eq!(part_a(include_str!("example.txt")), 13);
    // assert_eq!(part_a(include_str!("input.txt")), 0);
    // assert_eq!(part_b(include_str!("example.txt")), 0);
    // assert_eq!(part_b(include_str!("input.txt")), 0);
}

fn get_next_part(input: &str) -> &str {
    if input.starts_with('[') && input.ends_with(']') {
        &input[1..input.len()]
    }
}

fn test_pairs(pair: &(&str, &str)) -> bool {
    dbg!(pair);

    true
}

fn part_a(input: &str) -> usize {
    let mut index_sum = 0;

    for pair in input.trim().split("\n\n").enumerate() {
        let split = pair.1.split_once('\n').unwrap();

        if test_pairs(&split) {
            index_sum += pair.0;
        }
    }

    index_sum
}

// fn part_b(input: &str) -> i64 {
//     0
// }
