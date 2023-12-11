use std::collections::{HashMap, HashSet};
use rayon::prelude::*;

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data));
}

#[derive(Debug, PartialEq, Eq, Hash, Default)]
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
    let pattern:Vec<char> = process_pattern(lines.next().unwrap());
    lines.next();
    let nodes: HashMap<String, Node> = lines.map(|line| {
        process_nodes(line)
    }).collect();
    let mut curr_nodes:HashSet<&Node> = HashSet::new();
    for node in &nodes {
        if node.0.ends_with("A") {
            curr_nodes.insert(&node.1);
        }
    }
    let mut node_map: Vec<usize> = vec![];
    let mut steps = 0;
    let pattern_length = pattern.len();
    let starting_direction = pattern[0];
    for node in curr_nodes {
        steps = 0;
        let mut next_node = node;
        while true {
            let direction = pattern[steps % pattern_length];
            if direction == 'L' {
                let n = nodes.get(&next_node.left).unwrap();
                next_node = n;
            } else {
                let n = nodes.get(&next_node.right).unwrap();
                next_node = n;
            }
            steps += 1;
            if next_node.label.ends_with('Z') {
                break;
            }
        }
        node_map.push(steps);
    }
    dbg!(&node_map);
    steps = lcm(node_map);
    return steps;
}

fn lcm(nums: Vec<usize>) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(nums[1..].into());
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

fn process_nodes(line: &str) -> (String, Node){
    let mut line_split = line.split(' ');
    let label: String = line_split.next().unwrap().into();
    let label_ends_with: char = label.chars().last().unwrap().clone();
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
        let data = include_str!("test3.txt");
        assert_eq!(6, part_one(data));
    }

}
