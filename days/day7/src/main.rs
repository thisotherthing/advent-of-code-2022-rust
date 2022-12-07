fn main() {
    assert_eq!(part_a(include_str!("example.txt")), 95437);
    assert_eq!(part_a(include_str!("input.txt")), 1845346);
    assert_eq!(part_b(include_str!("example.txt")), 24933642);
    assert_eq!(part_b(include_str!("input.txt")), 3636703);
}

struct FsNode {
    name: String,
    parent: usize,
    children: Vec<usize>,
    size: usize,
}

impl std::fmt::Debug for FsNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "(name: {}, parent: {}, children : {:?}, size: {:?})",
            self.name, self.parent, self.children, self.size
        )
    }
}

// fn log_tree(index: usize, nodes: &Vec<FsNode>, depth: usize) {
//     let info = format!(
//         "{}{} ({})",
//         " ".repeat(depth),
//         nodes[index].name,
//         nodes[index].size
//     );
//     dbg!(info);
//     for child_index in 0..nodes[index].children.len() {
//         // dbg!(nodes[index].children[child_index]);
//         log_tree(nodes[index].children[child_index], nodes, depth + 1);
//     }
// }

fn add_child_sizes(index: usize, nodes: &mut Vec<FsNode>) -> usize {
    for child_index in 0..nodes[index].children.len() {
        // dbg!(nodes[index].children[child_index]);
        nodes[index].size += add_child_sizes(nodes[index].children[child_index], nodes);
    }

    nodes[index].size
}

fn parse_nodes(input: &str, nodes: &mut Vec<FsNode>) {
    let mut current_head = 0;

    for line in input.trim().lines() {
        if line.starts_with('$') {
            // command
            let data = line.split(' ').collect::<Vec<&str>>();

            if data[1] == "cd" {
                if data[2] == ".." {
                    current_head = nodes[current_head].parent;
                } else {
                    // add new node
                    let new_node = FsNode {
                        name: data[2].to_string(),
                        parent: current_head,
                        children: vec![],
                        size: 0,
                    };
                    let new_head = nodes.len();
                    if !nodes.is_empty() {
                        nodes[current_head].children.push(new_head);
                    }

                    current_head = new_head;
                    nodes.push(new_node);
                }
            }
        } else {
            // entry
            let data = line.trim().split_once(' ').unwrap();
            if data.0 != "dir" {
                nodes[current_head].size += data.0.parse::<usize>().expect("couldn't parse value");
            }
        }
    }
}

fn part_a(input: &str) -> usize {
    let mut nodes: Vec<FsNode> = vec![];

    parse_nodes(input, &mut nodes);

    // dbg!("{:?}", &nodes);
    add_child_sizes(0, &mut nodes);
    // log_tree(0, &nodes, 0);

    let mut sum_of_small_folders = 0;

    for node in nodes {
        if node.size <= 100000 {
            sum_of_small_folders += node.size
        }
    }

    sum_of_small_folders
}

fn part_b(input: &str) -> usize {
    let mut nodes: Vec<FsNode> = vec![];

    parse_nodes(input, &mut nodes);

    // dbg!("{:?}", &nodes);
    add_child_sizes(0, &mut nodes);
    // log_tree(0, &nodes, 0);

    let free_space = 70000000 - nodes[0].size;
    dbg!(&free_space);
    let space_to_free = 30000000 - free_space;
    dbg!(&space_to_free);

    nodes.sort_by(|a, b| a.size.partial_cmp(&b.size).unwrap());

    // dbg!(&nodes);

    let mut folder_to_delete_size = 0;

    for node in nodes {
        if node.size >= space_to_free {
            folder_to_delete_size = node.size;
            break;
        }
    }

    folder_to_delete_size
}
