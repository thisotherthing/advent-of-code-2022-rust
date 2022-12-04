fn main() {
    assert_eq!(part_a(include_str!("example.txt")), 2);
    assert_eq!(part_a(include_str!("input.txt")), 573);
    assert_eq!(part_b(include_str!("example.txt")), 4);
    assert_eq!(part_b(include_str!("input.txt")), 867);
}

struct Range(u32, u32);

fn get_range(input: &str) -> Range {
    let split = input.split_once('-').unwrap();

    Range(
        split.0.parse::<u32>().unwrap(),
        split.1.parse::<u32>().unwrap(),
    )
}

fn part_a(input: &str) -> u32 {
    let mut count: u32 = 0;

    for line in input.trim().lines() {
        let split = line.split_once(',').unwrap();

        let one = get_range(split.0);
        let two = get_range(split.1);

        if (one.0 <= two.0 && one.1 >= two.1) || (two.0 <= one.0 && two.1 >= one.1) {
            count += 1;
        }
    }

    count
}

fn part_b(input: &str) -> u32 {
    let mut count: u32 = 0;

    for line in input.trim().lines() {
        let split = line.split_once(',').unwrap();

        let one = get_range(split.0);
        let two = get_range(split.1);

        if (one.0 <= two.0 && one.1 >= two.1)
            || (two.0 <= one.0 && two.1 >= one.1)
            || (one.0 <= two.0 && one.0 <= two.1 && one.1 >= two.0)
            || (two.0 <= one.0 && two.1 <= one.1 && two.1 >= one.0)
        {
            count += 1;
        }
    }

    count
}
