use std::cmp::min;
use anyhow::Result;
use itertools::Itertools;
use rangemap::RangeMap;


fn source_to_dest(source_num: u64, range_map: &RangeMap<u64, u64>) -> u64 {
    match range_map.get_key_value(&source_num) {
        Some((range, dest_start)) => dest_start + (source_num - range.start),
        None => source_num,
    }
}

fn parse_section_ranges(section: &str) -> RangeMap<u64, u64> {
    section
        .split('\n')
        .skip(1)
        .map(|row| {
            row.split(' ')
                .map(|elem| elem.parse::<u64>().expect("range elem is an integer"))
                .collect_tuple::<(u64, u64, u64)>().expect("rows should have exactly three integers")
        })
        .map(|(dest_start, source_start, range_len)| {
            (source_start..source_start+range_len, dest_start)
        })
        .collect()
}

pub fn process(input: &str) -> Result<String> {
    let sections: Vec<&str> = input.split("\n\n").collect();
    assert_eq!(sections.len(), 8);

    let starting_seeds = sections[0]
        .strip_prefix("seeds: ")
        .expect("first line lists the seeds")
        .split(' ')
        .map(|seed| seed.parse::<u64>().expect("seed part is numeric"));


    let seed_to_soil_map = parse_section_ranges(sections[1]);
    let soil_to_fertilizer_map = parse_section_ranges(sections[2]);
    let fertilizer_to_water_map = parse_section_ranges(sections[3]);
    let water_to_light_map = parse_section_ranges(sections[4]);
    let light_to_temperature_map = parse_section_ranges(sections[5]);
    let temperature_to_humidity_map = parse_section_ranges(sections[6]);
    let humidity_to_location_map = parse_section_ranges(sections[7]);

    let locations = starting_seeds
        .tuples::<(u64, u64)>()
        .flat_map(|(seed_start, range_length)| seed_start .. seed_start+range_length)
        .map(|seed_num| {
            let soil = source_to_dest(seed_num, &seed_to_soil_map);
            let fertilizer = source_to_dest(soil, &soil_to_fertilizer_map);
            let water = source_to_dest(fertilizer, &fertilizer_to_water_map);
            let light = source_to_dest(water, &water_to_light_map);
            let temperature = source_to_dest(light, &light_to_temperature_map);
            let humidity = source_to_dest(temperature, &temperature_to_humidity_map);
            source_to_dest(humidity, &humidity_to_location_map)
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
        assert_eq!("46", process(input)?);
        Ok(())
    }
}
