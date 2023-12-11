pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data));
}

fn part_one(data: &str) -> i64 {
    let mut readings: Vec<Vec<i64>> = vec![];
    for line in data.lines() {
        readings.push(read_line_to_vec(line));
    }
    let mut total_sum: i64 = 0;
    for reading in readings {
        let mut history: Vec<Vec<i64>> = process_down(reading);
        dbg!(&history);
        let prediction: i64 = process_up(history);
        total_sum += prediction;
    }

    total_sum
}

fn process_up(history: Vec<Vec<i64>>) -> i64 {
    let mut level = history.len() - 1;
    let mut prediction: i64 = 0;
    while true {
        prediction += history[level].last().unwrap();
        if level != 0 {
            level -= 1;
        } else {
            break;
        }
    }
    prediction
}

fn process_down(reading: Vec<i64>) -> Vec<Vec<i64>> {
    let mut history: Vec<Vec<i64>> = vec![];
    let mut level = reading.clone();
    history.push(reading);
    while !level.iter().all(|&x| x == 0) {
        level = process_single(level);
        history.push(level.clone());
    }
    history
}
fn process_single(reading: Vec<i64>) -> Vec<i64> {
    let mut next_level: Vec<i64> = vec![];
    for i in 0..(reading.len() - 1) {
        next_level.push(reading[i + 1] - reading[i]);
    }
    next_level
}

fn read_line_to_vec(line: &str) -> Vec<i64> {
    let mut reading: Vec<i64> = vec![];
    for val in line.split(' ') {
        reading.push(val.parse::<i64>().unwrap());
    }
    reading
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        assert_eq!(114, part_one(data));
    }

}
