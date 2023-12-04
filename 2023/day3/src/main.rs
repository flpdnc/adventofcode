pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data));
}

fn part_one(data: &str) -> usize {
    let schematic = build_schematic(data);
    let answer = process_schematic(schematic);
    return answer;
}

fn check_for_surrounding_symbols(schematic: Vec<Vec<char>>, digit_row: usize, col_range: (usize, usize)) -> bool {
    let mut drow_start: usize = 0;
    if digit_row != 0 {
       drow_start = digit_row - 1;
    }

    let mut drow_end: usize = schematic.len();
    if digit_row != drow_end - 1 {
        drow_end = digit_row + 2;
    }

    for r in drow_start..drow_end {
        let row = schematic[r].clone();
        let mut col_start: usize = 0;
        if col_range.0 != 0 {
            col_start = col_range.0 - 1;
        }

        let mut col_end: usize = row.len();
        if col_range.1 != col_end - 1 {
            col_end = col_range.1 + 2;
        }
        for c in col_start..col_end {
            let ch = row[c];
            dbg!(ch);
            if !ch.is_ascii_digit() && ch != '.' {
                return true;
            }
        }
    }
    return false;
}

fn check_for_surrounding_numbers(schematic: Vec<Vec<char>>, gear_row: usize, gear_col: usize) -> usize {
    let mut grow_start: usize = 0;
    if gear_row != 0 {
       grow_start = gear_row - 1;
    }

    let mut grow_end: usize = schematic.len();
    if gear_row != grow_end - 1 {
        grow_end = gear_row + 2;
    }

    let mut attached_nums: Vec<usize> = vec![];
    for r in grow_start..grow_end {
        let row = schematic[r].clone();
        let mut col_start: usize = 0;
        if gear_col != 0 {
            col_start = gear_col - 1;
        }
        let mut col_end: usize = row.len();
        if gear_col != col_end - 1 {
            col_end = gear_col + 2;
        }
        for c in col_start..col_end {
            let mut num: Vec<char> = vec![];
            let ch = row[c];
            if ch.is_ascii_digit() {
                let mut dig = c;
                while dig > 0 && row[dig-1].is_ascii_digit() {
                    dig -= 1;
                }
                let mut dig_start = dig;
                while dig_start < row.len() && row[dig_start].is_ascii_digit() {
                    num.push(row[dig_start]);
                    dig_start += 1;
                }
                let mut mult: usize = 1;
                let mut gear_num: usize = 0;
                num.reverse();
                for digit in num {
                    gear_num += digit.to_digit(10).unwrap() as usize * mult;
                    mult *= 10;
                }
                if gear_num != 0 && !attached_nums.contains(&gear_num) {
                    attached_nums.push(gear_num);
                }
            }
        }
    }
    dbg!(&attached_nums);
    if attached_nums.len() == 2 {
        attached_nums[0] * attached_nums[1]
    } else {
        0
    }
}

fn process_schematic(schematic: Vec<Vec<char>>) -> usize {
    let mut res: usize = 0;
    let mut r: usize = 0;
    let mut c: usize = 0;
    while r < schematic.len() {
        let row = schematic[r].clone();
        while c < row.len() {
            if row[c] == '*' {
                res += check_for_surrounding_numbers(schematic.clone(), r, c);
            }
            c += 1;
        }
        c = 0;
        r += 1;
    }
    res
}

fn build_schematic(data: &str) -> Vec<Vec<char>> {
    let mut schematic: Vec<Vec<char>> = vec![];
    for line in data.lines() {
        let mut row: Vec<char> = vec![];
        for c in line.chars() {
            row.push(c);
        }
        schematic.push(row);
    }
    schematic
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        assert_eq!(467835, part_one(data));
    }

}
