#[derive(Copy, Clone, Debug, PartialEq)]
enum Direction {
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    StartingPosition,
}

#[derive(Copy, Debug, Clone, PartialEq)]
struct Location {
    x: usize,
    y: usize,
    dir: Direction,
}

impl Location {
    fn new(x: usize, y: usize, dir: Direction) -> Self {
        Self {
            x, y, dir
        }
    }

    fn get_next_locs(&self, map: &DirectionMap) -> (Location, Location) {
        match self.dir {
            Direction::StartingPosition => {
                (Location::new(self.x, self.y + 1, map.get_dir_at_loc(self.x, self.y + 1)), 
                 Location::new(self.x, self.y - 1, map.get_dir_at_loc(self.x, self.y - 1)))
            },
            Direction::NorthSouth => {
                (Location::new(self.x, self.y + 1, map.get_dir_at_loc(self.x, self.y + 1)), 
                 Location::new(self.x, self.y - 1, map.get_dir_at_loc(self.x, self.y - 1)))
            },
            Direction::EastWest => {
                (Location::new(self.x + 1, self.y, map.get_dir_at_loc(self.x + 1, self.y)), 
                 Location::new(self.x - 1, self.y, map.get_dir_at_loc(self.x - 1, self.y)))
            },
            Direction::NorthEast => {
                (Location::new(self.x, self.y - 1, map.get_dir_at_loc(self.x, self.y - 1)), 
                 Location::new(self.x + 1, self.y, map.get_dir_at_loc(self.x + 1, self.y)))
            },
            Direction::NorthWest => {
                (Location::new(self.x, self.y - 1, map.get_dir_at_loc(self.x, self.y - 1)), 
                 Location::new(self.x - 1, self.y, map.get_dir_at_loc(self.x - 1, self.y)))
            },
            Direction::SouthWest => {
                (Location::new(self.x - 1, self.y, map.get_dir_at_loc(self.x - 1, self.y)), 
                 Location::new(self.x, self.y + 1, map.get_dir_at_loc(self.x, self.y + 1)))
            },
            Direction::SouthEast => {
                (Location::new(self.x + 1, self.y, map.get_dir_at_loc(self.x + 1, self.y)), 
                 Location::new(self.x, self.y + 1, map.get_dir_at_loc(self.x, self.y + 1)))
            },
            _ => {
                (Location::new(0, 0, Direction::Ground), 
                 Location::new(0, 0, Direction::Ground))
            }
        }
    }
}

struct DirectionMap {
    map: Vec<Vec<Direction>>,
}

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data));
}

fn part_one(data: &str) -> usize {
    let map: DirectionMap = DirectionMap::new(data);
    let start_location: Location = find_start(&map);
    // Input start can only go North or South to start. S is a vertical Pipe.
    let mut path: Vec<Location> = vec![start_location.clone()];
    // Hard coding for test, will update for live input
    //let mut curr_location: Location = Location::new(start_location.x + 1, start_location.y, map.get_dir_at_loc(start_location.x + 1, start_location.y));
    // Hard coding for live input
    let mut curr_location: Location = start_location.get_next_locs(&map).0;
    path.push(curr_location.clone());
    let mut previous_loc = path[0].clone();
    while curr_location != start_location {
        let (loc1, loc2) = curr_location.get_next_locs(&map);
        if loc1 == previous_loc {
            previous_loc = curr_location.clone();
            curr_location = loc2;
        } else {
            previous_loc = curr_location.clone();
            curr_location = loc1;
        }
        path.push(curr_location.clone());
//        dbg!(&path);
    }
    path.len() / 2
}

fn find_start(map: &DirectionMap) -> Location {
    let mut x: usize = 0;
    let mut y: usize = 0;
    while y < map.map.len() {
        let row = &map.map[y];
        while x < row.len() {
            if row[x] == Direction::StartingPosition {
                return Location::new(x, y, Direction::StartingPosition);
            }
            x += 1;
        }
        y += 1;
        x = 0;
    }
    Location::new(x, y, Direction::Ground)
}

impl DirectionMap {
    fn new(data: &str) -> Self {
        let map = data.lines().into_iter().map(|y| {
            y.chars().into_iter().map(|x| {
                match x {
                    '|' => Direction::NorthSouth,
                    '-' => Direction::EastWest,
                    'L' => Direction::NorthEast,
                    'J' => Direction::NorthWest,
                    '7' => Direction::SouthWest,
                    'F' => Direction::SouthEast,
                    'S' => Direction::StartingPosition,
                     _  => Direction::Ground,
                    }
                }).collect()}
            ).collect();
        Self {
            map
        }
    }

    fn get_dir_at_loc(&self, x: usize, y: usize) -> Direction {
        self.map[y][x]
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        assert_eq!(4, part_one(data));
    }

    #[test]
    fn two() {
        let data = include_str!("test2.txt");
        assert_eq!(8, part_one(data));
    }

//    #[test]
//    fn two() {
//        let data = include_str!("test.txt");
//        assert_eq!(, part_two(data));
//    }
}
