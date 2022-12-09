use std::collections::HashSet;

#[derive(Eq, Hash, PartialEq, Copy, Clone)]
struct Point(i64, i64);

fn main() {
    assert_eq!(part_a(include_str!("example.txt")), 13);
    assert_eq!(part_a(include_str!("input.txt")), 6175);
    assert_eq!(part_b(include_str!("example.txt")), 1);
    assert_eq!(part_b(include_str!("example_large.txt")), 36);
    assert_eq!(part_b(include_str!("input.txt")), 2578);
}

fn print_bridge(head: Point, tail: Point) {
    let bounds_x = 6;
    let bounds_y = 5;

    for y in (0..bounds_y).rev() {
        for x in 0..bounds_x {
            if x == head.0 && y == head.1 {
                print!("H");
            } else if x == tail.0 && y == tail.1 {
                print!("T");
            } else if x == 0 && y == 0 {
                print!("s");
            } else {
                print!(".");
            }
        }

        println!();
    }

    println!();
}

fn tail_step(head: &Point, tail: &mut Point) {
    // step tail
    let pos_diff = Point(head.0 - tail.0, head.1 - tail.1);

    if head.0 != tail.0 && head.1 != tail.1 && (pos_diff.0.abs() > 1 || pos_diff.1.abs() > 1) {
        tail.0 += pos_diff.0.clamp(-1, 1);
        tail.1 += pos_diff.1.clamp(-1, 1);
    } else if pos_diff.0.abs() > 1 {
        tail.0 += pos_diff.0.clamp(-1, 1);
    } else if pos_diff.1.abs() > 1 {
        tail.1 += pos_diff.1.clamp(-1, 1);
    }
}

fn part_a(input: &str) -> usize {
    let mut head = Point(0, 0);
    let mut tail = Point(0, 0);

    let mut visited: HashSet<Point> = HashSet::new();

    let mut amount: i64;
    let mut dir = Point(0, 0);

    for line in input.trim().lines() {
        // println!("{}", &line);

        let split = line.split_once(' ').unwrap();
        amount = split.1.parse::<i64>().unwrap();

        dir.0 = 0;
        dir.1 = 0;

        match split.0 {
            "U" => {
                dir.1 = 1;
            }
            "D" => {
                dir.1 = -1;
            }
            "L" => {
                dir.0 = -1;
            }
            "R" => {
                dir.0 = 1;
            }
            _ => {}
        }

        for _ in 0..amount {
            // step head
            head.0 += dir.0;
            head.1 += dir.1;

            tail_step(&head, &mut tail);

            // print_bridge(head, tail);

            // track tail position
            visited.insert(Point(tail.0, tail.1));
        }
    }

    visited.len()
}

fn rope_step(rope: &mut Vec<Point>, head_index: usize) {
    // step tail
    let pos_diff = Point(
        rope[head_index].0 - rope[head_index + 1].0,
        rope[head_index].1 - rope[head_index + 1].1,
    );

    if rope[head_index].0 != rope[head_index + 1].0
        && rope[head_index].1 != rope[head_index + 1].1
        && (pos_diff.0.abs() > 1 || pos_diff.1.abs() > 1)
    {
        rope[head_index + 1].0 += pos_diff.0.clamp(-1, 1);
        rope[head_index + 1].1 += pos_diff.1.clamp(-1, 1);
    } else if pos_diff.0.abs() > 1 {
        rope[head_index + 1].0 += pos_diff.0.clamp(-1, 1);
    } else if pos_diff.1.abs() > 1 {
        rope[head_index + 1].1 += pos_diff.1.clamp(-1, 1);
    }
}

fn part_b(input: &str) -> usize {
    let mut rope: Vec<Point> = vec![];

    for _ in 0..10 {
        rope.push(Point(0, 0));
    }

    let mut visited: HashSet<Point> = HashSet::new();

    let mut amount: i64;
    let mut dir = Point(0, 0);

    for line in input.trim().lines() {
        // println!("{}", &line);

        let split = line.split_once(' ').unwrap();
        amount = split.1.parse::<i64>().unwrap();

        dir.0 = 0;
        dir.1 = 0;

        match split.0 {
            "U" => {
                dir.1 = 1;
            }
            "D" => {
                dir.1 = -1;
            }
            "L" => {
                dir.0 = -1;
            }
            "R" => {
                dir.0 = 1;
            }
            _ => {}
        }

        for _ in 0..amount {
            // step head
            rope[0].0 += dir.0;
            rope[0].1 += dir.1;

            for i in 0..9 {
                rope_step(&mut rope, i);
            }

            // track tail position
            visited.insert(Point(rope[9].0, rope[9].1));

            // print_bridge(head, tail);
        }
    }

    visited.len()
}
