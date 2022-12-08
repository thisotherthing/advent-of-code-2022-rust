fn main() {
    assert_eq!(part_a(include_str!("example.txt")), 21);
    assert_eq!(part_a(include_str!("input.txt")), 1717);
    assert_eq!(part_b(include_str!("example.txt")), 8);
    assert_eq!(part_b(include_str!("input.txt")), 321975);
}

#[derive(Debug)]
struct Forest {
    width: usize,
    height: usize,
    trees: Vec<u8>,
}

impl Forest {
    fn get_height(&self, x: usize, y: usize) -> u8 {
        self.trees[y * self.width + x]
    }
}

fn load_forest(input: &str) -> Forest {
    let width = input.split_once('\n').unwrap().0.len();
    let height = input.trim().lines().count();

    let mut trees = Vec::with_capacity(width * height);

    for line in input.trim().lines() {
        for c in line.chars() {
            trees.push(c.to_digit(10).unwrap() as u8);
        }
    }

    Forest {
        width,
        height,
        trees,
    }
}

fn tree_is_visible(x: usize, y: usize, forest: &Forest) -> bool {
    // edges are visible
    if x == 0 || x == forest.width - 1 || y == 0 || y == forest.height - 1 {
        return true;
    }

    let tree_height = forest.get_height(x, y);

    let mut hidden = 0;

    // check up
    for iy in 0..y {
        if forest.get_height(x, iy) >= tree_height {
            hidden += 1;
            break;
        }
    }
    // down
    for iy in y + 1..forest.height {
        if forest.get_height(x, iy) >= tree_height {
            hidden += 1;
            break;
        }
    }

    // check left
    for ix in 0..x {
        if forest.get_height(ix, y) >= tree_height {
            hidden += 1;
            break;
        }
    }
    // check right
    for ix in x + 1..forest.width {
        if forest.get_height(ix, y) >= tree_height {
            hidden += 1;
            break;
        }
    }

    hidden < 4
}

fn walk(start: (usize, usize), direction: (i64, i64), forest: &Forest) -> i64 {
    let tree_height = forest.get_height(start.0, start.1);

    let mut distance = 0;

    let mut position = (start.0 as i64 + direction.0, start.1 as i64 + direction.1);

    let max_x: i64 = (forest.width - 1).try_into().unwrap();
    let max_y: i64 = (forest.height - 1).try_into().unwrap();

    while position.0 >= 0 && position.0 <= max_x && position.1 >= 0 && position.1 <= max_y {
        distance += 1;

        if forest.get_height(position.0 as usize, position.1 as usize) >= tree_height {
            break;
        }

        position.0 += direction.0;
        position.1 += direction.1;
    }

    distance
}

fn get_scenic_score(x: usize, y: usize, forest: &Forest) -> i64 {
    let mut score = 1;

    // check up
    score *= walk((x, y), (0, 1), forest);

    // down
    score *= walk((x, y), (0, -1), forest);

    // check left
    score *= walk((x, y), (-1, 0), forest);

    // check right
    score *= walk((x, y), (1, 0), forest);

    score
}

fn part_a(input: &str) -> i64 {
    let mut visible_trees_count = 0;

    let forest = load_forest(input);

    for y in 0..forest.height {
        for x in 0..forest.width {
            if tree_is_visible(x, y, &forest) {
                visible_trees_count += 1;
                // dbg!(format!(" {} ", forest.trees[get_index(x, y, forest.width)]));
            } else {
                // dbg!(format!("[{}]", forest.trees[get_index(x, y, forest.width)]));
            }
        }
    }

    // dbg!("{:?}", &forest);

    visible_trees_count
}

fn part_b(input: &str) -> i64 {
    let mut highest_score = 0;

    let forest = load_forest(input);

    for y in 0..forest.height {
        for x in 0..forest.width {
            highest_score = highest_score.max(get_scenic_score(x, y, &forest));
        }
    }

    highest_score
}
