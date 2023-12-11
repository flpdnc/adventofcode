pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data));
}

#[derive(Debug, Clone)]
struct Galaxy {
    x: usize,
    y: usize,

}

impl Galaxy {
    fn new(x: usize, y: usize) -> Self {
        Self {
            x, y
        }
    }
}

fn expand_universe(data: &str) -> Vec<Vec<usize>> {
    let mut space_map: Vec<Vec<usize>> = vec![];
    let mut expanded_univ: Vec<Vec<usize>> = vec![];
    // Process from strings
    for l in data.lines() {
        let mut space_row: Vec<usize> = vec![];
        for c in l.chars() {
            match c {
                '#' => space_row.push(0),
                _ => space_row.push(1),
            };
        }
        space_map.push(space_row);
    }
    // Expand rows
    let mut row_expanded_univ: Vec<Vec<usize>> = vec![];
    for row in space_map {
        if row.iter().all(|x| x != &0) {
            for _ in 0..1000000 {
                row_expanded_univ.push(row.clone());
            }
        }
        row_expanded_univ.push(row.clone());
    }
    // Expand columns
    let row_size: usize = row_expanded_univ[0].len();
    let mut c: usize = 0;
    while c < row_size {
        // Copy each column into the expanded universe as a row, this will rotate by 90 degrees
        let col_exp = row_expanded_univ.clone().iter().map(|r| r[c]).all(|x| x != 0);
        if col_exp {
            expanded_univ.push(row_expanded_univ.clone().iter().map(|r| r[c]).collect());
        }
        expanded_univ.push(row_expanded_univ.clone().iter().map(|r| r[c]).collect());
        c += 1;
    }
    expanded_univ
}

fn get_galaxy_coords(space_map: Vec<Vec<char>>) -> Vec<Galaxy> {
    let mut galaxy_list: Vec<Galaxy> = vec![];
    // Galaxy is now flipped 90 degrees, so rows are are y cols, and cols are x rows
    for y in 0..space_map.len() {
        for x in 0..space_map[y].len() {
            match space_map[y][x] {
                '#' => galaxy_list.push(Galaxy::new(x, y)),
                _ => (),
            }
        }
    }
    galaxy_list
}

fn calculate_distance_sum(galaxy_list: Vec<Galaxy>) -> usize {
    let mut total_distance: usize = 0;
    for i in 0..galaxy_list.len() - 1 {
        let mut distance: usize = 0;
        for j in i + 1..galaxy_list.len() {
            let g1 = &galaxy_list[i];
            let g2 = &galaxy_list[j];
            if g1.x >= g2.x {
                distance += g1.x - g2.x;
            } else {
                distance += g2.x - g1.x;
            }
            if g1.y >= g2.y {
                distance += g1.y - g2.y;
            } else {
                distance += g2.y - g1.y;
            }
        }
        total_distance += distance;

    }
    total_distance
}

fn part_one(data: &str) -> usize {
    let space_map: Vec<Vec<char>> = expand_universe(&data);
    let galaxy_list = get_galaxy_coords(space_map);
    let total_distance = calculate_distance_sum(galaxy_list);
    total_distance
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        assert_eq!(374, part_one(data));
    }
}
