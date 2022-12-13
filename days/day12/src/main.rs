fn main() {
    assert_eq!(part_a(include_str!("example.txt")), 31);
    assert_eq!(part_a(include_str!("input.txt")), 504);
    assert_eq!(part_b(include_str!("example.txt")), 29);
    assert_eq!(part_b(include_str!("input.txt")), 0);
}

#[derive(Debug)]
struct Board {
    start_idx: usize,
    end_idx: usize,
    heights: Vec<u64>,
    width: usize,
    height: usize,
}

impl Board {
    fn get_idx_from_pos(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }
    fn get_pos(&self, idx: usize) -> (usize, usize) {
        (idx % self.width, idx / self.width)
    }
}

fn read_board(input: &str) -> Board {
    let width = input.split_once('\n').unwrap().0.len();
    let height = input.trim().lines().count();

    let mut heights: Vec<u64> = vec![];

    let mut start_idx = 0;
    let mut end_idx = 0;

    let mut idx = 0;

    for line in input.trim().lines() {
        for c in line.chars() {
            match c {
                'S' => {
                    start_idx = idx;
                    heights.push('a' as u64);
                }
                'E' => {
                    end_idx = idx;
                    heights.push('z' as u64);
                }
                _ => {
                    heights.push(c as u64);
                }
            }

            idx += 1;
        }
    }

    Board {
        start_idx,
        end_idx,
        width,
        height,
        heights,
    }
}

fn update_pos(
    x: usize,
    y: usize,
    board: &Board,
    costs: &mut Vec<usize>,
    touched: &mut Vec<usize>,
    cost: usize,
    height: u64,
) {
    let idx = board.get_idx_from_pos(x, y);
    if cost + 1 < costs[idx] && height + 1 >= board.heights[idx] {
        costs[idx] = cost + 1;

        if !touched.contains(&idx) {
            touched.push(idx);
        }
    }
}

fn check_neighbors(idx: usize, board: &Board, costs: &mut Vec<usize>, touched: &mut Vec<usize>) {
    let pos = board.get_pos(idx);
    // up
    if pos.1 > 0 {
        update_pos(
            pos.0,
            pos.1 - 1,
            board,
            costs,
            touched,
            costs[idx],
            board.heights[idx],
        );
    }

    // down
    if pos.1 < board.height - 1 {
        update_pos(
            pos.0,
            pos.1 + 1,
            board,
            costs,
            touched,
            costs[idx],
            board.heights[idx],
        );
    }

    // left
    if pos.0 > 0 {
        update_pos(
            pos.0 - 1,
            pos.1,
            board,
            costs,
            touched,
            costs[idx],
            board.heights[idx],
        );
    }

    // right
    if pos.0 < board.width - 1 {
        update_pos(
            pos.0 + 1,
            pos.1,
            board,
            costs,
            touched,
            costs[idx],
            board.heights[idx],
        );
    }
}

fn get_shortest_cost(start_idx: usize, board: &Board) -> usize {
    let mut costs = vec![usize::MAX - 2; board.heights.len()];

    let mut was_touched: Vec<Vec<usize>> = vec![vec![], vec![]];
    let mut curr_idx = 0;
    let mut prev_idx = 1 - curr_idx;

    // initialize data
    costs[start_idx] = 0;
    was_touched[prev_idx].push(start_idx);

    loop {
        // check neighbors of touched elements, if lower steps is possible
        // up, down, left, right

        for i in (0..was_touched[prev_idx].len()).rev() {
            check_neighbors(
                was_touched[prev_idx][i],
                board,
                &mut costs,
                &mut was_touched[curr_idx],
            );
        }

        // end if all were checked
        if was_touched[curr_idx].is_empty() {
            break;
        }

        was_touched[prev_idx].clear();

        // ping pong buffers
        curr_idx = 1 - curr_idx;
        prev_idx = 1 - prev_idx;
    }

    costs[board.end_idx]
}

fn part_a(input: &str) -> usize {
    let board = read_board(input);

    get_shortest_cost(board.start_idx, &board)
}

fn part_b(input: &str) -> usize {
    let board = read_board(input);

    let start_height = 'a' as u64;

    board
        .heights
        .iter()
        .enumerate()
        .filter_map(|f| {
            if f.1 == &start_height {
                Option::Some(f.0)
            } else {
                Option::None
            }
        })
        .map(|f| get_shortest_cost(f, &board))
        .min()
        .unwrap()
}
