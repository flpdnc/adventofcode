#[derive(Debug)]
struct TargetRange {
    dest_start: usize,
    source_start: usize,
    range_length: usize,
}

impl TargetRange {
    fn new(dest_start: usize, source_start: usize, range_length: usize) -> Self {
        Self {
            dest_start,
            source_start,
            range_length,
        }
    }
    
    fn in_range(self: &Self, seed: usize) -> usize {
        let source = self.source_start as i64;
        let dest = self.dest_start as i64;
        let offset = dest - source;
        if seed >= self.source_start && seed < self.source_start + self.range_length {
            return (seed as i64 + offset) as usize;
        }
        return seed;
    }
}

pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data));
}

fn part_one(data: &str) -> usize {
    dbg!(usize::MAX);
    let mut l: usize = 0;
    let lines: Vec<&str> = data.lines().collect();
    let mut seeds: Vec<usize> = vec![];
    let mut seed_to_soil: Vec<TargetRange> = vec![];
    let mut soil_to_fertilizer: Vec<TargetRange> = vec![];
    let mut fertilizer_to_water: Vec<TargetRange> = vec![];
    let mut water_to_light: Vec<TargetRange> = vec![];
    let mut light_to_temperature: Vec<TargetRange> = vec![];
    let mut temperature_to_humidity: Vec<TargetRange> = vec![];
    let mut humidity_to_location: Vec<TargetRange> = vec![];
    while l < lines.len() {
        if lines[l].starts_with("seeds:") {
            seeds = process_seeds(lines[l]);
        }
        if lines[l].starts_with("seed-to-soil") {
            l += 1;
            while !lines[l].is_empty() {
                seed_to_soil.push(process_ranges(lines[l]));
                l += 1;
            }
        }
        if lines[l].starts_with("soil-to-fertilizer") {
            l += 1;
            while !lines[l].is_empty() {
                soil_to_fertilizer.push(process_ranges(lines[l]));
                l += 1;
            }
        }
        if lines[l].starts_with("fertilizer-to-water") {
            l += 1;
            while !lines[l].is_empty() {
                fertilizer_to_water.push(process_ranges(lines[l]));
                l += 1;
            }
        }
        if lines[l].starts_with("water-to-light") {
            l += 1;
            while !lines[l].is_empty() {
                water_to_light.push(process_ranges(lines[l]));
                l += 1;
            }
        }
        if lines[l].starts_with("light-to-temperature") {
            l += 1;
            while !lines[l].is_empty() {
                light_to_temperature.push(process_ranges(lines[l]));
                l += 1;
            }
        }
        if lines[l].starts_with("temperature-to-humidity") {
            l += 1;
            while !lines[l].is_empty() {
                temperature_to_humidity.push(process_ranges(lines[l]));
                l += 1;
            }
        }
        if !lines[l].is_empty() {
            if !lines[l].starts_with("#") && !lines[l].starts_with("seed") && !lines[l].starts_with("hum") {
                humidity_to_location.push(process_ranges(lines[l]));
            }
        }
        l += 1;
    }
    let mut destination_locations: Vec<usize> = vec![];
    for seed in seeds {
        dbg!(&seed);
        // Convert to soil location
        let mut soil_loc: usize = seed;
        for soil in &seed_to_soil {
           soil_loc = soil.in_range(seed);
           if soil_loc != seed {
               break;
           }
        }
        dbg!(&soil_loc);
        // Convert to fert location
        let mut fert_loc: usize = soil_loc;
        dbg!(&soil_to_fertilizer);
        for fert in &soil_to_fertilizer {
           fert_loc = fert.in_range(soil_loc);
           if fert_loc != soil_loc {
               break;
           }
        }
        dbg!(&fert_loc);
        // Convert to water location
        let mut water_loc: usize = fert_loc;
        for water in &fertilizer_to_water {
           water_loc = water.in_range(fert_loc);
           if water_loc != fert_loc {
               break;
           }
        }
        dbg!(&water_loc);
        // Convert to light location
        let mut light_loc: usize = water_loc;
        for light in &water_to_light {
           light_loc = light.in_range(water_loc);
           if light_loc != water_loc {
               break;
           }
        }
        dbg!(&light_loc);
        // Convert to temp location
        let mut temp_loc: usize = light_loc;
        for temp in &light_to_temperature {
           temp_loc = temp.in_range(light_loc);
           if temp_loc != light_loc {
               break;
           }
        }
        dbg!(&temp_loc);
        // Convert to hum location
//        dbg!(&temperature_to_humidity);
        let mut hum_loc: usize = temp_loc;
        for hum in &temperature_to_humidity {
           hum_loc = hum.in_range(temp_loc);
           if hum_loc != temp_loc {
               break;
           }
        }
        dbg!(&hum_loc);
        // Convert to location
        let mut dest_loc: usize = hum_loc;
//        dbg!(&humidity_to_location);
        for location in &humidity_to_location {
           dest_loc = location.in_range(hum_loc);
           if dest_loc != hum_loc {
               break;
           }
        }
        dbg!(&dest_loc);
        destination_locations.push(dest_loc);
    }
    dbg!(&destination_locations);

    destination_locations.sort();
    return *destination_locations.first().unwrap();
}

fn process_seeds(line: &str) -> Vec<usize> {
    let mut seeds: Vec<usize> = vec![];
    for i in line.split(' ') {
        if !i.starts_with("seeds") {
            seeds.push(i.parse::<usize>().unwrap());
        }
    }
    return seeds;
}

fn process_ranges(line: &str) -> TargetRange {
    let mut range_split = line.split(' ');
    let dest_start = range_split.next().unwrap().parse::<usize>().unwrap();
    let source_start = range_split.next().unwrap().parse::<usize>().unwrap();
    let range_length = range_split.next().unwrap().parse::<usize>().unwrap();
    return TargetRange::new(
        dest_start,
        source_start,
        range_length,
    );
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        assert_eq!(35, part_one(data));
    }

}
