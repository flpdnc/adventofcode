use criterion::{Criterion, criterion_group, criterion_main};
use day6_2015::{LightMatrix, InstructionSet, count_on_by_rows};

use std::{fs::read_to_string, sync::Arc};

pub fn bench_lights(c: &mut Criterion) {
    let mut lights = LightMatrix::new();
    let file_input = read_to_string("input.txt").unwrap();
    for line in file_input.lines() {
        let instruction_set = InstructionSet::new(line.to_string());
        lights.control_lights(instruction_set);
    }
    //let lights = Arc::new(lights.lights);
    c.bench_function("Bench Lights", |b| b.iter(|| {
//        let lights = Arc::clone(&lights);
//        let _ = count_on_by_rows(lights);
        let _ = lights.count_on();
    }));
}
criterion_group!(benches, bench_lights);
criterion_main!(benches);
