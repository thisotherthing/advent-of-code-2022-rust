fn main() {
    assert_eq!(part_a(include_str!("example.txt")), 24000);
    assert_eq!(part_a(include_str!("input.txt")), 67027);
    assert_eq!(part_b(include_str!("example.txt")), 45000);
    assert_eq!(part_b(include_str!("input.txt")), 197291);
}

fn part_a(input: &str) -> i64 {
    let mut highest_amount: i64 = 0;
    let mut amount: i64 = 0;

    for line in input.trim().split('\n') {
        let line = line.trim();

        if !line.is_empty() {
            amount += line.parse::<i64>().unwrap();
        } else {
            highest_amount = highest_amount.max(amount);
            amount = 0;
        }
    }
    highest_amount = highest_amount.max(amount);

    highest_amount
}

struct Elf {
    calories: u64,
}

fn add_elf(elfs: &mut Vec<Elf>, amount: u64) {
    let elf = Elf { calories: amount };
    elfs.push(elf);
}

fn part_b(input: &str) -> u64 {
    let mut elfs: Vec<Elf> = vec![];

    let mut amount: u64 = 0;

    for line in input.trim().split('\n') {
        let line = line.trim();

        if !line.is_empty() {
            amount += line.parse::<u64>().unwrap();
        } else {
            add_elf(&mut elfs, amount);
            amount = 0;
        }
    }
    add_elf(&mut elfs, amount);

    elfs.sort_by(|a, b| b.calories.partial_cmp(&a.calories).unwrap());

    let mut top_three_calories_count = 0;
    for elf in elfs.iter_mut().take(3) {
        top_three_calories_count += elf.calories;
    }

    top_three_calories_count
}
