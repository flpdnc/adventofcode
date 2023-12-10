use colored::Colorize;

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
                // Set pipe type based off surrounding pipe options
                let mut west_x: usize = self.x;
                if self.x != 0 {
                    west_x -= 1;
                }
                let mut north_y: usize = self.y;
                if self.y != 0 {
                    north_y -= 1;
                }
                let west = map.get_dir_at_loc(west_x, self.y);
                let east = map.get_dir_at_loc(self.x + 1, self.y);
                let north = map.get_dir_at_loc(self.x, north_y);
                let south = map.get_dir_at_loc(self.x, self.y + 1);
                if west == Direction::EastWest || west == Direction::NorthWest || west == Direction::SouthWest {
                    if north == Direction::SouthWest || north == Direction::SouthEast || north == Direction::NorthSouth {
                        // Goes from west to north, so return NorthWest pipe type
                        return (Location::new(self.x, self.y - 1, map.get_dir_at_loc(self.x, self.y - 1)), 
                         Location::new(self.x - 1, self.y, map.get_dir_at_loc(self.x - 1, self.y)));
                    }
                    if east == Direction::SouthWest || east == Direction::EastWest || east == Direction::NorthWest {
                        // Goest west to east, so return EastWest pipe type
                        return (Location::new(self.x + 1, self.y, map.get_dir_at_loc(self.x + 1, self.y)), 
                         Location::new(self.x - 1, self.y, map.get_dir_at_loc(self.x - 1, self.y)));
                    }
                    if south == Direction::NorthEast || south == Direction::NorthWest || south == Direction::NorthSouth {
                        // Goes west to south, so return SouthWest Pipe type
                        return (Location::new(self.x - 1, self.y, map.get_dir_at_loc(self.x - 1, self.y)),
                         Location::new(self.x, self.y + 1, map.get_dir_at_loc(self.x, self.y + 1)));
                    }
                }
                if north == Direction::SouthWest || north == Direction::SouthEast || north == Direction::NorthSouth {
                    if east == Direction::SouthWest || east == Direction::EastWest || east == Direction::NorthWest {
                        // Goest north to east, so return NorthEast pipe type
                        return (Location::new(self.x, self.y - 1, map.get_dir_at_loc(self.x, self.y - 1)), 
                         Location::new(self.x + 1, self.y, map.get_dir_at_loc(self.x + 1, self.y)));
                    }
                    if south == Direction::NorthEast || south == Direction::NorthWest || south == Direction::NorthSouth {
                        // Goes north -> south, return NorthSouth Pipe type
                        return (Location::new(self.x, self.y + 1, map.get_dir_at_loc(self.x, self.y + 1)), 
                         Location::new(self.x, self.y - 1, map.get_dir_at_loc(self.x, self.y - 1)));
                    }
                }
                if east == Direction::SouthWest || east == Direction::EastWest || east == Direction::NorthWest {
                    if south == Direction::NorthEast || south == Direction::NorthWest || south == Direction::NorthSouth {
                        // Goes east to south, so return SouthEast Pipe type
                        return (Location::new(self.x + 1, self.y, map.get_dir_at_loc(self.x + 1, self.y)), 
                         Location::new(self.x, self.y + 1, map.get_dir_at_loc(self.x, self.y + 1)));
                    }
                }
                (Location::new(self.x, self.y + 1, map.get_dir_at_loc(self.x, self.y + 1)), 
                 Location::new(self.x, self.y - 1, map.get_dir_at_loc(self.x, north_y)))
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
    }
    let interior = print_map(map, path);
    interior
}

fn print_map(map: DirectionMap, path: Vec<Location>) -> usize {
    let mut x: usize = 0;
    let mut y: usize = 0;
    let mut interior_segments = 0;
    while y < map.map.len() {
        let row = &map.map[y];
        let mut wall_count: usize = 0;
        while x < row.len() -1 {
            let dir = map.get_dir_at_loc(x, y);
            if dir == Direction::NorthSouth && path.contains(&Location::new(x, y, dir)) {
                wall_count += 1;
                print!("{}", "║".bold().green());
            } else if dir == Direction::EastWest && path.contains(&Location::new(x, y, dir)) {
                print!("{}", "═".bold().green());
            } else if dir == Direction::NorthEast && path.contains(&Location::new(x, y, dir)) {
                wall_count += 1;
                print!("{}", "╚".bold().green());
            } else if dir == Direction::NorthWest && path.contains(&Location::new(x, y, dir)) {
                wall_count += 1;
                print!("{}", "╝".bold().green());
            } else if dir == Direction::SouthWest && path.contains(&Location::new(x, y, dir)) {
                print!("{}", "╗".bold().green());
            } else if dir == Direction::SouthEast && path.contains(&Location::new(x, y, dir)) {
                print!("{}", "╔".bold().green());
            } else if dir == Direction::StartingPosition && path.contains(&Location::new(x, y, dir)) {
                if Location::new(x, y, dir).get_next_locs(&map).1.dir == Direction::EastWest {
                    print!("{}", "╔".bold().green());
                } else {
                    wall_count += 1;
                    print!("{}", "║".bold().green());
                }
            } else if !path.contains(&Location::new(x, y, dir)) {
                if wall_count % 2 == 1 {
                    print!("{}", "#".bold().blue());
                    interior_segments += 1;
                } else {
                    print!("{}", "#".bold().red());
                }
            }
            x += 1;
        }
        println!();
        y += 1;
        x = 0;
    }
    interior_segments
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
        let data = include_str!("test3.txt");
        assert_eq!(4, part_one(data));
    }

    #[test]
    fn two() {
        let data = include_str!("test4.txt");
        assert_eq!(8, part_one(data));
    }

    #[test]
    fn three() {
        let data = include_str!("test5.txt");
        assert_eq!(10, part_one(data));
    }
}
