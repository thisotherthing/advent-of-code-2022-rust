use std::str::FromStr;

fn main() {
    assert_eq!(monkey_business(include_str!("example.txt"), 3, 20), 10605);
    assert_eq!(monkey_business(include_str!("input.txt"), 3, 20), 58322);
    assert_eq!(
        monkey_business(include_str!("example.txt"), 1, 10000),
        2713310158
    );
    assert_eq!(
        monkey_business(include_str!("input.txt"), 1, 10000),
        13937702909
    );
}

#[derive(Debug)]
enum Operation {
    Add,
    Multiply,
}

#[derive(Debug)]
struct Monkey {
    items: Vec<u64>,
    operation: (u64, Operation, u64),
    test_division_val: u64,
    if_divisible: usize,
    if_not_divisible: usize,
    num_inspections: u64,
}

fn get_value_at_end<T: FromStr>(line: &str) -> Result<T, T::Err> {
    line.trim().split(' ').last().unwrap().parse::<T>()
}

fn monkey_business(input: &str, worry_decrease: u64, rounds: usize) -> u64 {
    let mut monkeys: Vec<Monkey> = vec![];

    // load monkeys
    for monkey_data in input.trim().split("Monkey ").filter(|&s| !s.is_empty()) {
        let mut lines = monkey_data.trim().lines();

        // ignore index
        lines.next();

        let items: Vec<u64> = lines
            .next()
            .unwrap()
            .trim()
            .split_once(':')
            .unwrap()
            .1
            .split(',')
            .map(|s| s.trim().parse::<u64>().unwrap())
            .collect();

        let operation_data: Vec<&str> = lines
            .next()
            .unwrap()
            .trim()
            .split_once("Operation: new = ")
            .unwrap()
            .1
            .split(' ')
            .collect();

        let operation = (
            if operation_data[0] == "old" {
                999
            } else {
                operation_data[0].parse::<u64>().unwrap()
            },
            if operation_data[1] == "+" {
                Operation::Add
            } else {
                Operation::Multiply
            },
            if operation_data[2] == "old" {
                999
            } else {
                operation_data[2].parse::<u64>().unwrap()
            },
        );

        let test_division_val = get_value_at_end::<u64>(lines.next().unwrap()).unwrap();

        let if_divisible = get_value_at_end::<usize>(lines.next().unwrap()).unwrap();
        let if_not_divisible = get_value_at_end::<usize>(lines.next().unwrap()).unwrap();

        let monkey = Monkey {
            items,
            operation,
            test_division_val,
            if_divisible,
            if_not_divisible,
            num_inspections: 0,
        };

        // dbg!("{:?}", &monkey);

        monkeys.push(monkey);
    }

    let dividers_products: u64 = monkeys
        .iter()
        .map(|monkey| monkey.test_division_val)
        .product();

    // run monkey inspections
    let mut item_val;
    let mut operation_left;
    let mut operation_right;
    for _ in 0..rounds {
        for monkey_idx in 0..monkeys.len() {
            for item_idx in (0..monkeys[monkey_idx].items.len()).rev() {
                item_val = monkeys[monkey_idx].items[item_idx];

                // set operation values
                operation_left = if monkeys[monkey_idx].operation.0 == 999 {
                    item_val
                } else {
                    monkeys[monkey_idx].operation.0
                };
                operation_right = if monkeys[monkey_idx].operation.2 == 999 {
                    item_val
                } else {
                    monkeys[monkey_idx].operation.2
                };

                match monkeys[monkey_idx].operation.1 {
                    Operation::Add => {
                        item_val = operation_left + operation_right;
                    }
                    Operation::Multiply => {
                        item_val = operation_left * operation_right;
                    }
                }

                item_val /= worry_decrease;

                item_val %= dividers_products;

                let target = if item_val % monkeys[monkey_idx].test_division_val == 0 {
                    monkeys[monkey_idx].if_divisible
                } else {
                    monkeys[monkey_idx].if_not_divisible
                };
                monkeys[target].items.push(item_val);

                monkeys[monkey_idx].items.remove(item_idx);

                monkeys[monkey_idx].num_inspections += 1;
            }
        }
    }

    // dbg!("{:?}", &monkeys);

    // sort to get most active monkeys
    monkeys.sort_by(|a, b| b.num_inspections.partial_cmp(&a.num_inspections).unwrap());

    monkeys[0].num_inspections * monkeys[1].num_inspections
}
