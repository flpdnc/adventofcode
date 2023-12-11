pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data));
}

fn part_one(data: &str) -> usize {
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

}
