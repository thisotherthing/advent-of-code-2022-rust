use std::collections::HashSet;

fn main() {
    assert_eq!(part_a(include_str!("example.txt")), 24);
    assert_eq!(part_a(include_str!("input.txt")), 1078);
    assert_eq!(part_b(include_str!("example.txt")), 93);
    assert_eq!(part_b(include_str!("input.txt")), 30157);
}

struct World {
    rocks: HashSet<(i64, i64)>,
    sand: HashSet<(i64, i64)>,
    x_bounds: (i64, i64),
    y_bounds: (i64, i64),
}

impl World {
    fn is_clear(&self, x: i64, y: i64) -> bool {
        let point = &(x, y);

        !self.rocks.contains(point) && !self.sand.contains(point)
    }

    fn is_clear_with_floor(&self, x: i64, y: i64) -> bool {
        if y > self.y_bounds.1 + 1 {
            return false;
        }

        let point = &(x, y);

        !self.rocks.contains(point) && !self.sand.contains(point)
    }
}

fn _log_world(world: &World) {
    for y in world.y_bounds.0 - 2..world.y_bounds.1 + 2 {
        for x in world.x_bounds.0 - 2..world.x_bounds.1 + 2 {
            let pos = &(x, y);
            if world.rocks.contains(pos) {
                print!("#");
            } else if world.sand.contains(pos) {
                print!("o");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn part_a(input: &str) -> i64 {
    // add rocks to map
    let mut world = World {
        rocks: HashSet::new(),
        sand: HashSet::new(),
        x_bounds: (i64::MAX, i64::MIN),
        y_bounds: (i64::MAX, i64::MIN),
    };

    // read rock instructions
    for line in input.trim().lines() {
        let points: Vec<(i64, i64)> = line
            .split(" -> ")
            .map(|f| {
                let split = f.split_once(',').unwrap();

                let point = (
                    split.0.parse::<i64>().unwrap(),
                    split.1.parse::<i64>().unwrap(),
                );

                world.x_bounds.0 = world.x_bounds.0.min(point.0);
                world.x_bounds.1 = world.x_bounds.1.max(point.0);
                world.y_bounds.0 = world.y_bounds.0.min(point.1);
                world.y_bounds.1 = world.y_bounds.1.max(point.1);

                point
            })
            .collect();

        // dbg!(&points);

        for window in points.windows(2) {
            // dbg!(&window);

            if window[0].0 != window[1].0 {
                // x
                for x in window[0].0.min(window[1].0)..=window[0].0.max(window[1].0) {
                    world.rocks.insert((x, window[0].1));
                }
            } else {
                // y
                for y in window[0].1.min(window[1].1)..=window[0].1.max(window[1].1) {
                    world.rocks.insert((window[0].0, y));
                }
            }
        }
    }
    // log_world(&world);

    let mut num_rounds = 0;

    let mut sand_pos: (i64, i64) = (0, 0);

    'outer: loop {
        sand_pos.0 = 500;
        sand_pos.1 = 0;

        loop {
            // check if sandcorn is outside bounds
            if sand_pos.1 > world.y_bounds.1 {
                break 'outer;
            }

            if world.is_clear(sand_pos.0, sand_pos.1 + 1) {
                // fall down
                sand_pos.1 += 1;
            } else if world.is_clear(sand_pos.0 - 1, sand_pos.1 + 1) {
                // fall down left
                sand_pos.0 -= 1;
                sand_pos.1 += 1;
            } else if world.is_clear(sand_pos.0 + 1, sand_pos.1 + 1) {
                // fall down right
                sand_pos.0 += 1;
                sand_pos.1 += 1;
            } else {
                // sand settled
                world.sand.insert((sand_pos.0, sand_pos.1));
                break;
            }
        }

        num_rounds += 1;
        // log_world(&world);
    }

    num_rounds
}

fn part_b(input: &str) -> i64 {
    // add rocks to map
    let mut world = World {
        rocks: HashSet::new(),
        sand: HashSet::new(),
        x_bounds: (i64::MAX, i64::MIN),
        y_bounds: (i64::MAX, i64::MIN),
    };

    // read rock instructions
    for line in input.trim().lines() {
        let points: Vec<(i64, i64)> = line
            .split(" -> ")
            .map(|f| {
                let split = f.split_once(',').unwrap();

                let point = (
                    split.0.parse::<i64>().unwrap(),
                    split.1.parse::<i64>().unwrap(),
                );

                world.x_bounds.0 = world.x_bounds.0.min(point.0);
                world.x_bounds.1 = world.x_bounds.1.max(point.0);
                world.y_bounds.0 = world.y_bounds.0.min(point.1);
                world.y_bounds.1 = world.y_bounds.1.max(point.1);

                point
            })
            .collect();

        // dbg!(&points);

        for window in points.windows(2) {
            // dbg!(&window);

            if window[0].0 != window[1].0 {
                // x
                for x in window[0].0.min(window[1].0)..=window[0].0.max(window[1].0) {
                    world.rocks.insert((x, window[0].1));
                }
            } else {
                // y
                for y in window[0].1.min(window[1].1)..=window[0].1.max(window[1].1) {
                    world.rocks.insert((window[0].0, y));
                }
            }
        }
    }
    // log_world(&world);

    let mut num_rounds = 0;

    let mut sand_pos: (i64, i64) = (0, 0);

    'outer: loop {
        sand_pos.0 = 500;
        sand_pos.1 = 0;

        loop {
            // check if sandcorn is outside bounds
            if !world.is_clear(500, 0) {
                break 'outer;
            }

            if world.is_clear_with_floor(sand_pos.0, sand_pos.1 + 1) {
                // fall down
                sand_pos.1 += 1;
            } else if world.is_clear_with_floor(sand_pos.0 - 1, sand_pos.1 + 1) {
                // fall down left
                sand_pos.0 -= 1;
                sand_pos.1 += 1;
            } else if world.is_clear_with_floor(sand_pos.0 + 1, sand_pos.1 + 1) {
                // fall down right
                sand_pos.0 += 1;
                sand_pos.1 += 1;
            } else {
                // sand settled
                world.sand.insert((sand_pos.0, sand_pos.1));
                break;
            }
        }

        num_rounds += 1;
        // log_world(&world);
    }

    num_rounds
}
