use std::collections::HashSet;

fn main() {
    assert_eq!(part_a(include_str!("example.txt")), 157);
    assert_eq!(part_a(include_str!("input.txt")), 7889);
    assert_eq!(part_b(include_str!("example.txt")), 70);
    assert_eq!(part_b(include_str!("input.txt")), 2825);
}

fn char_to_int(input: char) -> i64 {
    let mut code = input as u64;

    if code >= 96 {
        code -= 96;
    } else {
        code -= 38
    }
    code as i64
}

fn part_a(input: &str) -> i64 {
    let mut priorities_sum: i64 = 0;

    let mut items: HashSet<i64>;

    for line in input.trim().lines() {
        let length = line.len();

        items = HashSet::from_iter(line.chars().take(length / 2).map(char_to_int));

        for i in (length / 2)..line.len() {
            let char = line.chars().nth(i).unwrap();
            let code = char_to_int(char);
            if items.contains(&code) {
                // dbg!(char);
                priorities_sum += code;
                break;
            }
        }
    }

    priorities_sum
}

fn part_b(input: &str) -> i64 {
    let mut priorities_sum: i64 = 0;

    let mut items: HashSet<char> = HashSet::new();

    for (i, line) in input.trim().lines().enumerate() {
        // set hashset for new group
        if i % 3 == 0 {
            items = HashSet::from_iter(line.chars());
        } else {
            let set: HashSet<char> = HashSet::from_iter(line.chars());
            items.retain(|c| set.contains(c));
        }

        // get badge at the end
        if i % 3 == 2 {
            let badge_char = &items.iter().next().unwrap().to_owned();
            // dbg!(badge_char);
            priorities_sum += char_to_int(badge_char.to_owned());
        }
    }

    priorities_sum
}
