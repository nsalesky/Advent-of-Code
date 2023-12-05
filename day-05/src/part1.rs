use std::cmp::min;
use std::collections::HashMap;
use anyhow::Result;

fn process_ranges<'a>(lines: impl Iterator<Item=&'a str>) -> HashMap<u32, u32> {
    let mut map = HashMap::new();

    for line in lines {
        let components: Vec<u32> = line
            .split(' ')
            .map(|item| item.parse::<u32>().expect("range component is an integer"))
            .collect();
        assert_eq!(components.len(), 3);
        for delta in 0..components[2] {
            map.insert(components[1] + delta, components[0] + delta);
        }
    }

    map
}

pub fn process(input: &str) -> Result<String> {
    let sections: Vec<&str> = input.split("\n\n").collect();
    assert_eq!(sections.len(), 8);

    let starting_seeds: Vec<u32> = sections[0]
        .strip_prefix("seeds: ")
        .expect("first line lists the seeds")
        .split(' ')
        .map(|seed| seed.parse::<u32>().expect("seed is numeric"))
        .collect();

    let soil_to_fertilizer_map = process_ranges(
        sections[2]
            .split('\n')
            .skip(1));

    let fertilizer_to_water_map = process_ranges(
        sections[3]
            .split('\n')
            .skip(1));

    let water_to_light_map = process_ranges(
        sections[4]
            .split('\n')
            .skip(1));

    let light_to_temperature_map = process_ranges(
        sections[5]
            .split('\n')
            .skip(1));

    let temperature_to_humidity_map = process_ranges(
        sections[6]
            .split('\n')
            .skip(1));

    let humidity_to_location_map = process_ranges(
        sections[7]
            .split('\n')
            .skip(1));

    let locations = starting_seeds
        .iter()
        .map(|seed| {
            let fertilizer = soil_to_fertilizer_map.get(seed).unwrap_or(seed);
            let water = fertilizer_to_water_map.get(fertilizer).unwrap_or(fertilizer);
            let light = water_to_light_map.get(water).unwrap_or(water);
            let temperature = light_to_temperature_map.get(light).unwrap_or(light);
            let humidity = temperature_to_humidity_map.get(temperature).unwrap_or(temperature);
            humidity_to_location_map.get(humidity).unwrap_or(humidity)
        });

    let min_location = locations.reduce(min).expect("there is at least one location");

    Ok(min_location.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<()> {
        let input = "\
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        assert_eq!("35", process(input)?);
        Ok(())
    }
}