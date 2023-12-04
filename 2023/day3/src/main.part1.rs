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

fn process_schematic(schematic: Vec<Vec<char>>) -> usize {
    let mut res: usize = 0;
    let mut r: usize = 0;
    let mut c: usize = 0;
    while r < schematic.len() {
        let row = schematic[r].clone();
        while c < row.len() {
            let c_start: usize = c;
            let mut digit_chars: Vec<char> = vec![];
            if row[c].is_ascii_digit() {
                while c < row.len() && row[c].is_ascii_digit() {
                    digit_chars.push(row[c]);
                    c += 1;
                }
                dbg!(&digit_chars);
                if check_for_surrounding_symbols(schematic.clone(), r, (c_start, c - 1)) {
                    let mut mult: usize = 1;
                    let mut part_num: usize = 0;
                    digit_chars.reverse();
                    for digit in digit_chars {
                        part_num += digit.to_digit(10).unwrap() as usize * mult;
                        mult *= 10;
                    }
                    res += part_num;
                }
            } else {
                c += 1;
            }
        }
        c = 0;
        r += 1;
        dbg!(r);
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
        assert_eq!(4361, part_one(data));
    }

}
