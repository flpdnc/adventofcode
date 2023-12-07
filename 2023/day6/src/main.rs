pub fn main() {
    //Time:        45     98     83     73
    //Distance:   295   1734   1278   1210
    let mut data: Vec<(usize, usize)> = vec![];
    data.push((45, 295));
    data.push((98, 1734));
    data.push((83, 1278));
    data.push((73, 1210));
    println!("Part 1: {}", part_one(data));
    println!("Part 2: {}", check_race((45988373, 295173412781210)));
}

fn part_one(races: Vec<(usize, usize)>) -> usize {
    let mut wins: usize = 1;
    for race in races {
        wins *= check_race(race);
    }
    return wins;
}

fn check_race(race: (usize, usize)) -> usize {
    let mut wins: usize = 0;
    for i in 0..race.0 {
        let distance = (race.0 - i) * i;
        if distance > race.1 {
            wins += 1;
        }
    }
    return wins;
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data: (usize, usize) = (7, 9);
        assert_eq!(4, check_race(data));
    }

    #[test]
    fn two() {
        let data: (usize, usize) = (15, 40);
        assert_eq!(8, check_race(data));
    }

    #[test]
    fn three() {
        let data: (usize, usize) = (30, 200);
        assert_eq!(9, check_race(data));
    }

    #[test]
    fn all() {
        //Time:      7  15   30
        //Distance:  9  40  200
        let data: Vec<(usize, usize)> = vec![(7, 9),(15, 40), (30, 200)];
        assert_eq!(288, part_one(data));
    }

//    #[test]
//    fn two() {
//        let data = include_str!("test.txt");
//        assert_eq!(, part_two(data));
//    }
}
