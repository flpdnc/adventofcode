use std::{fs::read_to_string, sync::Arc};//, Mutex}};
use async_std::task;

use futures::{future::join_all, lock::Mutex};

fn main() {
    let mut lights = LightMatrix::new();
    let file_input = read_to_string("input.txt").unwrap();
    for line in file_input.lines() {
        let instruction_set = InstructionSet::new(line.to_string());
        lights.control_lights(instruction_set);
    }
    let total_on = lights.count_on();
    println!("{}", total_on);
}

#[derive(Debug)]
pub struct LightMatrix {
    pub lights: Vec<Vec<usize>>,
}

impl LightMatrix {
    pub fn new() -> LightMatrix {
        let mut lights: Vec<Vec<usize>> = vec![];
        for _ in 0..1000 {
            let mut row = vec![];
            for _ in 0..1000 {
                row.push(0);
            }
            lights.push(row);
        }
        LightMatrix { lights }
    }

    pub fn control_lights(&mut self, ins: InstructionSet) -> () {
        let control = ins.control.to_string();
        let on = "on".to_string();
        let off = "off".to_string();
        let toggle = "toggle".to_string();
        let mut con = 0;
        if control == off {
            con = 0;
        } else if control == on {
            con = 1;
        } else if control == toggle {
            con = 2;
        }
        for r in ins.start.x..(ins.end.x + 1) {
            for c in ins.start.y..(ins.end.y + 1) {
                match &con {
                    0 => {
                        if self.lights[r as usize][c as usize] > 0 {
                            self.lights[r as usize][c as usize] -= 1;
                        }
                    },
                    1 => {
                        self.lights[r as usize][c as usize] += 1;
                    },
                    2 => {
                        self.lights[r as usize][c as usize] += 2;
                    },
                    _ => {}
                }
            }
        }
    }
    
    pub fn count_on(&self) -> usize {
        let mut counter = 0;
        for r in 0..1000 {
            for c in 0..1000 {
                let col: usize = c as usize;
                let row: usize = r as usize;
                counter += &self.lights[row][col];
            }
        }
        counter
    }
}

pub async fn count_on_by_rows(lights: Arc<Vec<Vec<usize>>>) -> usize {
    let counter = Arc::new(Mutex::new(0));
    let mut tasks = vec![];
    for r in 0..100 {
        //let row_counter = Arc::clone(&counter);
        for r_offset in 0..10 {
            let counter = Arc::clone(&counter);
            let lights = Arc::clone(&lights);
            //let row = r as usize + r_offset as usize;
            let row = Arc::new(r as usize + r_offset as usize);
            let row = Arc::clone(&row);
            let handle = task::spawn(
                async { count_on_in_row(lights, row, counter) }
            );
            tasks.push(handle);
        }
    }
    let _ = join_all(tasks);
    let count = *counter.lock().await;
    count
}

pub async fn count_on_in_row(lights: Arc<Vec<Vec<usize>>>, row: Arc<usize>, counter: Arc<Mutex<usize>> ) -> () {
    let mut row_count = 0;
    let row: usize = *row;
    for c in 0..1000 {
        let col: usize = c as usize;
        row_count += lights[row][col];
    }
    let mut counter = counter.lock().await;
    *counter += row_count as usize;
}

    

#[derive(Debug)]
pub struct Coord {
    pub x: u32,
    pub y: u32,
}

impl Coord {
    pub fn new(coord: String) -> Coord {
        if coord == String::new() {
            return Coord {
                x: 0,
                y: 0,
            };
        }
        let coords: Vec<u32> = coord.split(",").map(|c| c.parse().unwrap()).collect();
        let x = coords[0]; 
        let y = coords[1];
        Coord { x, y }
    }
}

#[derive(Debug)]
pub struct InstructionSet {
    pub control: String,
    pub start: Coord,
    pub end: Coord,
}

impl InstructionSet {
    pub fn new(instructions: String) -> InstructionSet {
        let inst_pieces = instructions.split(" ");
        let mut control: String = String::new();
        let mut start: Coord = Coord::new(String::new());
        let mut end: Coord = Coord::new(String::new());
        for word in inst_pieces {
            if word.to_string() == "turn".to_string() || word.to_string() == "through".to_string() {
                continue;
            }
            else if word.to_string() == "toggle".to_string() ||
                    word.to_string() == "on".to_string() ||
                    word.to_string() == "off".to_string() {
                        control = word.to_string();
                        continue
                    }
            else if word.contains(",") {
                let tmp_coord = Coord::new(word.to_string());
                if tmp_coord.x > end.x {
                    start = end;
                    end = tmp_coord;
                }
                else {
                    start = tmp_coord;
                }
            }
        }
        InstructionSet { 
            control, 
            start,
            end,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let mut lights = LightMatrix::new();
        let instruction_set = InstructionSet::new("turn on 0,0 through 0,0".to_string());
        lights.control_lights(instruction_set);
        let total_on = lights.count_on();
        assert_eq!(total_on, 1);
    }

    #[test]
    fn test_two() {
        let mut lights = LightMatrix::new();
        let instruction_set = InstructionSet::new("toggle 0,0 through 999,999".to_string());
        lights.control_lights(instruction_set);
        let total_on = lights.count_on();
        assert_eq!(total_on, 2000000);
    }
/*
    use test::Bencher;

    #[bench]
    fn bench_lights(b: &mut Bencher) {
        let mut lights = LightMatrix::new();
        let file_input = read_to_string("input.txt").unwrap();
        for line in file_input.lines() {
            let instruction_set = InstructionSet::new(line.to_string());
            lights.control_lights(instruction_set);
        }
        b.iter(|| {
            lights.count_on();
        })
    }

    #[bench]
    fn bench_lights_async(b: &mut Bencher) {
        let mut lights = LightMatrix::new();
        let file_input = read_to_string("input.txt").unwrap();
        for line in file_input.lines() {
            let instruction_set = InstructionSet::new(line.to_string());
            lights.control_lights(instruction_set);
        }
        let lights = Arc::new(lights.lights);
        b.iter(|| {
            let lights = Arc::clone(&lights);
            let total_on = count_on_by_rows(lights);
        })
    }




    #[test]
    fn test_one() {
        let mut lights = LightMatrix::new();
        let instruction_set = InstructionSet::new("turn on 0,0 through 999,999".to_string());
        lights.control_lights(instruction_set);
        let total_on = lights.count_on();
        assert_eq!(total_on, 1000000);
    }

    #[test]
    fn test_two() {
        let mut lights = LightMatrix::new();
        let instruction_set = InstructionSet::new("toggle 0,0 through 999,000".to_string());
        lights.control_lights(instruction_set);
        let total_on = lights.count_on();
        assert_eq!(total_on, 1000);
    }

    #[test]
    fn test_three() {
        let mut lights = LightMatrix::new();
        println!("{}", lights.count_on());
        let instruction_set = InstructionSet::new("turn on 0,0 through 999,999".to_string());
        lights.control_lights(instruction_set);
        println!("{}", lights.count_on());
        
        let instruction_set = InstructionSet::new("turn off 499,499 through 500,500".to_string());
        println!("{:?}", instruction_set);
        lights.control_lights(instruction_set);
        println!("{}", lights.count_on());
        let total_on = lights.count_on();
        assert_eq!(total_on, 999996);
    }
*/


}

