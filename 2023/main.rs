pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data));
}

fn part_one(data: &str) -> usize {
    data.lines().map(read_line).map(|(m, c)| m - c).sum()
}

fn read_line(line: &str) -> (usize, usize) {
    unimplemented!();
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        assert_eq!(, part_one(data));
    }

//    #[test]
//    fn two() {
//        let data = include_str!("test.txt");
//        assert_eq!(, part_two(data));
//    }
}
