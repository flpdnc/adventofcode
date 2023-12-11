use std::collections::HashMap;

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data));
}

#[derive(Debug)]
struct Node {
    label: String,
    left: String,
    right: String,
}

impl Node {
    fn new(label: String, left: String, right: String) -> Self {
        Self {
            label, left, right
        }
    }
}

fn part_one(data: &str) -> usize {
    let mut lines = data.lines();
    let mut pattern:Vec<char> = process_pattern(lines.next().unwrap());
    lines.next();
    let nodes: HashMap<String, Node> = lines.map(|line| {
        process_nodes(line)
    }).collect();
    let mut curr_node = nodes.get("AAA").unwrap();
    let mut steps = 0;
    while curr_node.label != "ZZZ" {
        steps += 1;
        let direction = pattern[0];
        pattern = pattern[1..].into();
        if direction == 'L' {
            curr_node = nodes.get(&curr_node.left).unwrap();
        } else if direction == 'R' {
            curr_node = nodes.get(&curr_node.right).unwrap();
        }
        pattern.push(direction);

    }
    return steps;
}

fn process_nodes(line: &str) -> (String, Node){
    let mut line_split = line.split(' ');
    let label: String = line_split.next().unwrap().into();
    line_split.next();
    let left: String = line_split.next().unwrap()[1..4].into();
    let right: String = line_split.next().unwrap()[0..3].into();
    (label.clone(), Node::new(
        label,
        left,
        right,
    ))
}

fn process_pattern(line: &str) -> Vec<char> {
    line.chars().collect()
}



#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        assert_eq!(2, part_one(data));
    }

    #[test]
    fn two() {
        let data = include_str!("test2.txt");
        assert_eq!(6, part_one(data));
    }
}
