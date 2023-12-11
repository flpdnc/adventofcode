use std::{collections::{HashMap, HashSet, BTreeMap}, time::Instant};
use rayon::prelude::*;

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data));
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Node {
    label_ends_with: u8,
    left: (u8, u8, u8),
    right: (u8, u8, u8),
}

impl Node {
    fn new(label_ends_with: u8, left: (u8, u8, u8), right: (u8, u8, u8)) -> Self {
        Self {
            label_ends_with, left, right
        }
    }
}

fn part_one(data: &str) -> usize {
    let mut lines = data.lines();
    let pattern:Vec<u8> = process_pattern(lines.next().unwrap());
    lines.next();
    let nodes: HashMap<(u8, u8, u8), Node> = lines.map(|line| {
        process_nodes(line)
    }).collect();
    let mut curr_nodes:HashSet<&Node> = HashSet::new();
    for node in &nodes {
        if node.0.2 == 'A' as u8 {
            curr_nodes.insert(&node.1);
        }
//        if curr_nodes.len() > 2 {
//            break;
//        }
    }
    let mut steps = 0;
    let mut found_all_end_z = false;
    let pattern_length = pattern.len();
    let ending = 'Z' as u8;
    let left = b'L';
    while !found_all_end_z {
        let start = Instant::now();
        steps += 1;
        if pattern[steps % pattern_length] == left {
            curr_nodes = curr_nodes.iter().map(|x| nodes.get(&x.left).unwrap()).collect();
            found_all_end_z = curr_nodes.iter().all(|x| x.label_ends_with == ending);
        } else {
            curr_nodes = curr_nodes.iter().map(|x| nodes.get(&x.right).unwrap()).collect();
            found_all_end_z = curr_nodes.iter().all(|x| x.label_ends_with == ending);
        }
        let elapsed = start.elapsed();
//        dbg!(elapsed.as_nanos());
        if steps % 10000 == 0 {
            dbg!(elapsed.as_nanos());
            dbg!(&steps);
        }
//        if curr_nodes.par_iter().any(|x| x.label_ends_with == 'Z') {
//            dbg!(&curr_nodes);
//        }
    }
    return steps;
}

fn process_nodes(line: &str) -> ((u8, u8, u8), Node){
    let mut line_split = line.split(' ');
    let label: Vec<u8> = line_split.next().unwrap().into();
    let label_ends_with: u8 = *label.last().unwrap();
    line_split.next();
    let left: Vec<u8> = line_split.next().unwrap()[1..4].into();
    let right: Vec<u8> = line_split.next().unwrap()[0..3].into();
    ((label[0], label[1], label[2]), Node::new(
        label_ends_with,
        (left[0], left[1], left[2]),
        (right[0], right[1], right[2]),
    ))
}

fn process_pattern(line: &str) -> Vec<u8> {
    line.bytes().collect()
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
