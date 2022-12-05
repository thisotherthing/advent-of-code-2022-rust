fn main() {
    assert_eq!(part_a(include_str!("example.txt")), "CMZ");
    assert_eq!(part_a(include_str!("input.txt")), "TDCHVHJTG");
    assert_eq!(part_b(include_str!("example.txt")), "MCD");
    assert_eq!(part_b(include_str!("input.txt")), "NGCMPJLHV");
}

enum ParseState {
    Stacks,
    Moves,
}

fn part_a(input: &str) -> String {
    let lines = input.trim_end().lines();

    let num_stacks = (lines.clone().take(1).next().unwrap().len() + 1) / 4;

    let mut stacks: Vec<Vec<char>> = vec![];

    for _ in 0..num_stacks {
        stacks.push(vec![]);
    }

    // dbg!(num_stacks);

    let mut parse_state = ParseState::Stacks;

    for line in lines {
        // dbg!(line);
        match parse_state {
            ParseState::Stacks => {
                if line.trim().is_empty() {
                    parse_state = ParseState::Moves;
                    // dbg!(&stacks);
                } else if line.contains('[') {
                    let line_chars = line.chars().collect::<Vec<char>>();
                    for idx in 0..num_stacks {
                        if line_chars[idx * 4] == '[' {
                            stacks[idx].push(line_chars[idx * 4 + 1]);
                        }
                    }
                }
            }
            ParseState::Moves => {
                let move_data: Vec<usize> = line
                    .split(' ')
                    .filter_map(|v| v.parse::<usize>().ok())
                    .collect();

                let count = move_data[0];
                let from = move_data[1] - 1;
                let to = move_data[2] - 1;

                // dbg!(move_data);
                for _ in 0..count {
                    let tmp = &stacks[from].remove(0);
                    stacks[to].insert(0, *tmp);
                }
                // dbg!(&stacks);
            }
        }
    }

    let mut solution = "".to_string();

    for stack in stacks {
        solution.push(*stack.first().unwrap());
    }
    // dbg!(&solution);

    solution
}

fn part_b(input: &str) -> String {
    let lines = input.trim_end().lines();

    let num_stacks = (lines.clone().take(1).next().unwrap().len() + 1) / 4;

    let mut stacks: Vec<Vec<char>> = vec![];

    for _ in 0..num_stacks {
        stacks.push(vec![]);
    }

    // dbg!(num_stacks);

    let mut parse_state = ParseState::Stacks;

    for line in lines {
        // dbg!(line);
        match parse_state {
            ParseState::Stacks => {
                if line.trim().is_empty() {
                    parse_state = ParseState::Moves;
                    // dbg!(&stacks);
                } else if line.contains('[') {
                    let line_chars = line.chars().collect::<Vec<char>>();
                    for idx in 0..num_stacks {
                        if line_chars[idx * 4] == '[' {
                            stacks[idx].push(line_chars[idx * 4 + 1]);
                        }
                    }
                }
            }
            ParseState::Moves => {
                let move_data: Vec<usize> = line
                    .split(' ')
                    .filter_map(|v| v.parse::<usize>().ok())
                    .collect();

                let count = move_data[0];
                let from = move_data[1] - 1;
                let to = move_data[2] - 1;

                // dbg!(move_data);
                for i in 0..count {
                    let tmp = &stacks[from].remove(0);
                    stacks[to].insert(i, *tmp);
                }
                // dbg!(&stacks);
            }
        }
    }

    let mut solution = "".to_string();

    for stack in stacks {
        solution.push(*stack.first().unwrap());
    }
    // dbg!(&solution);

    solution
}
