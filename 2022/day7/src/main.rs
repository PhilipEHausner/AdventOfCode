mod file_tree;

use file_tree::{FileTree, Node, NodeType};
use util::read_files::read_file_as_vector;

#[derive(PartialEq, Debug)]
enum Command {
    Change,
    List,
    ListEntry,
}

fn main() {
    let lines = read_file_as_vector("./files/day7.txt").expect("Error reading file.");
    println!("Solution part 1 {}", solve1(lines));
}

fn solve1(lines: Vec<String>) -> u64 {
    let tree = create_file_tree(&lines);
    tree.get_sub_100k_dir_sizes_sum()
}

fn create_file_tree(lines: &Vec<String>) -> FileTree {
    let root_node = Node::new(NodeType::Folder, "ROOT", 0);
    let mut current_path: Vec<String> = vec![];
    let mut tree = FileTree{root: root_node};

    let mut curr_index = 0;
    loop {
        if curr_index >= lines.len() { break; }
        let line = &lines[curr_index];
        let next_command = get_next_command(&line);

        if next_command == Command::Change {
            match &line[5..] {
                "/" => {
                    current_path = vec![];
                },
                ".." => {
                    current_path.pop();
                }
                _ => {
                    current_path.push(line[5..].to_string());
                }
            }
            curr_index += 1;
        } else if next_command == Command::List {
            loop {
                curr_index += 1;
                if curr_index >= lines.len() { break; }
                let list_line = &lines[curr_index];
                if &list_line[..3] == "dir" {
                    let new_child = Node { node_type: NodeType::Folder, name: list_line[4..].to_string(), size: 0, children: vec![] };
                    tree.append_child(&current_path, new_child);
                } else if &list_line[0..1] == "$" { 
                    break;
                } else {
                    let file_line = list_line.split(" ").collect::<Vec<&str>>();
                    let new_child = Node { node_type: NodeType::File, name: file_line[1].to_string(), size: file_line[0].parse::<u64>().unwrap(), children: vec![] };
                    tree.append_child(&current_path, new_child);
                }
            }
        } else {
            panic!("No valid command given.");
        }
    }

    tree.compute_sizes();
    tree
}

fn get_next_command(line: &str) -> Command {
    match &line[..4] {
        "$ cd" => { Command::Change },
        "$ ls" => { Command::List },
        _ => { Command::ListEntry },
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let lines = read_file_as_vector("./files/test.txt").expect("Error reading file.");
        assert_eq!(solve1(lines), 95437);
    }
}