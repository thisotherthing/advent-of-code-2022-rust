fn main() {
    assert_eq!(part_a("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4), 7);
    assert_eq!(part_a("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), 5);
    assert_eq!(part_a("nppdvjthqldpwncqszvftbrmjlhg", 4), 6);
    assert_eq!(part_a("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4), 10);
    assert_eq!(part_a("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4), 11);
    assert_eq!(part_a(include_str!("input.txt"), 4), 1987);
    assert_eq!(part_a("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14), 19);
    assert_eq!(part_a("bvwbjplbgvbhsrlpgdmjqwftvncz", 14), 23);
    assert_eq!(part_a("nppdvjthqldpwncqszvftbrmjlhg", 14), 23);
    assert_eq!(part_a("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14), 29);
    assert_eq!(part_a("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14), 26);
    assert_eq!(part_a(include_str!("input.txt"), 14), 3059);
}

fn part_a(input: &str, marker_len: usize) -> u64 {
    let size = marker_len;

    let mut first_unique: u64 = u64::MAX;

    for (index, chars) in input
        .chars()
        .collect::<Vec<char>>()
        .windows(size)
        .enumerate()
    {
        let mut data = vec![];

        for x in chars {
            data.push(x);
        }
        data.sort_unstable();

        if data.windows(2).all(|charpair| charpair[0] != charpair[1]) {
            first_unique = index as u64 + size as u64;
            break;
        }
    }

    first_unique
}
