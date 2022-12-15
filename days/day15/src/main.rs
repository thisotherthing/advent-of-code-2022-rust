use std::ops::Range;

use regex::Regex;

fn main() {
    assert_eq!(part_a(include_str!("example.txt"), 10), 26);
    assert_eq!(part_a(include_str!("input.txt"), 2000000), 4907780);
    assert_eq!(part_b(include_str!("example.txt"), 21), 56000011);
    assert_eq!(part_b(include_str!("input.txt"), 4000001), 13639962836448);
}
#[derive(Debug)]
struct Sensor {
    pos: (i64, i64),
    beacon: (i64, i64),
    range: i64,
}
impl Sensor {
    fn in_range(&self, x: i64, y: i64) -> bool {
        self.range >= (x - self.pos.0).abs() + (y - self.pos.1).abs()
    }
    fn is_beacon(&self, x: i64, y: i64) -> bool {
        self.beacon.0 == x && self.beacon.1 == y
    }
    fn _get_x_range(&self, range: i64) -> Range<usize> {
        (self.pos.0 - self.range).clamp(0, range) as usize
            ..(self.pos.0 + self.range + 1).clamp(0, range) as usize
    }
    fn get_y_range(&self, range: i64) -> Range<i64> {
        (self.pos.1 - self.range).clamp(0, range)..(self.pos.1 + self.range + 1).clamp(0, range)
    }
    fn get_x_start(&self, y: i64) -> i64 {
        self.pos.0 - (self.range - (self.pos.1 - y).abs()).max(0)
    }
    fn get_x_end(&self, y: i64) -> i64 {
        self.pos.0 + (self.range - (self.pos.1 - y).abs()).max(0)
    }
}

#[derive(Debug)]
struct Bounds {
    min: (i64, i64),
    max: (i64, i64),
}

impl Bounds {
    fn new() -> Bounds {
        Bounds {
            min: (i64::MAX, i64::MAX),
            max: (i64::MIN, i64::MIN),
        }
    }
}

fn part_a(input: &str, y: i64) -> i64 {
    let re = Regex::new(r"x=(-?[0-9]+).+y=(-?[0-9]+).+x=(-?[0-9]+).+y=(-?[0-9]+)").unwrap();

    let mut sensors: Vec<Sensor> = vec![];

    let mut bounds = Bounds::new();

    for line in input.trim().lines() {
        let caps = re.captures(line).unwrap();

        let values: Vec<i64> = caps
            .iter()
            .skip(1)
            .map(|m| m.unwrap().as_str().parse::<i64>().unwrap())
            .collect();

        let sensor = Sensor {
            pos: (values[0], values[1]),
            beacon: (values[2], values[3]),
            range: (values[0] - values[2]).abs() + (values[1] - values[3]).abs(),
        };

        bounds.min.0 = bounds.min.0.min(sensor.pos.0 - sensor.range - 1);
        bounds.min.1 = bounds.min.1.min(sensor.pos.1 - sensor.range - 1);

        bounds.max.0 = bounds.max.0.max(sensor.pos.0 + sensor.range + 1);
        bounds.max.1 = bounds.max.1.max(sensor.pos.1 + sensor.range + 1);

        sensors.push(sensor);

        // dbg!("{:?}", &values);
    }

    // dbg!(&sensors);
    // dbg!(&bounds);

    let mut num_positions = 0;

    for x in bounds.min.0..=bounds.max.0 {
        if sensors.iter().any(|sensor| sensor.in_range(x, y))
            && sensors.iter().all(|sensor| !sensor.is_beacon(x, y))
        {
            num_positions += 1;
        }
    }

    num_positions
}

fn part_b(input: &str, max_range: i64) -> i64 {
    let re = Regex::new(r"x=(-?[0-9]+).+y=(-?[0-9]+).+x=(-?[0-9]+).+y=(-?[0-9]+)").unwrap();

    let mut sensors: Vec<Sensor> = vec![];

    for line in input.trim().lines() {
        let caps = re.captures(line).unwrap();

        let values: Vec<i64> = caps
            .iter()
            .skip(1)
            .map(|m| m.unwrap().as_str().parse::<i64>().unwrap())
            .collect();

        sensors.push(Sensor {
            pos: (values[0], values[1]),
            beacon: (values[2], values[3]),
            range: (values[0] - values[2]).abs() + (values[1] - values[3]).abs(),
        });

        // dbg!("{:?}", &values);
    }

    // dbg!(&sensors);
    // dbg!(&bounds);

    let mut possiblilities = vec![];

    let ranges_y: Vec<Range<i64>> = sensors.iter().map(|s| s.get_y_range(max_range)).collect();

    for y in 0..=max_range {
        possiblilities.clear();

        for s in sensors.iter().enumerate() {
            if ranges_y[s.0].contains(&y) {
                let start = s.1.get_x_start(y) - 1;
                if start >= 0 && start <= max_range {
                    possiblilities.push(start);
                }

                let end = s.1.get_x_end(y) + 1;
                if end >= 0 && end <= max_range {
                    possiblilities.push(end);
                }
            }
        }

        // dbg!(&possiblilities);

        for x in &possiblilities {
            if sensors.iter().all(|sensor| !sensor.in_range(*x as i64, y)) {
                dbg!("{}", (x, y));

                return x * 4000000 + y;
            }
        }
    }

    0
}
