use std::cmp::min;
use anyhow::Result;

fn source_to_dest(source_num: u32, mappings: &Vec<&str>) -> u32 {
    for mapping in mappings {
        let components: Vec<u32> = mapping
            .split(' ')
            .map(|item: &str| item.parse().expect("range component is an integer"))
            .collect();
        assert_eq!(components.len(), 3);

        if source_num >= components[1] && source_num < components[1] + components[2] {
            let difference = source_num - components[1];
            return components[0] + difference;
        }
    }

    return source_num;
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

    let soil_to_fertilizer_map: Vec<&str> =
        sections[2]
            .split('\n')
            .skip(1)
            .collect();

    let fertilizer_to_water_map: Vec<&str> =
        sections[3]
            .split('\n')
            .skip(1)
            .collect();

    let water_to_light_map: Vec<&str> =
        sections[4]
            .split('\n')
            .skip(1)
            .collect();

    let light_to_temperature_map: Vec<&str> =
        sections[5]
            .split('\n')
            .skip(1)
            .collect();

    let temperature_to_humidity_map: Vec<&str> =
        sections[6]
            .split('\n')
            .skip(1)
            .collect();

    let humidity_to_location_map: Vec<&str> =
        sections[7]
            .split('\n')
            .skip(1)
            .collect();

    let locations = starting_seeds
        .iter()
        .map(move |seed| {
            let fertilizer = source_to_dest(*seed, &soil_to_fertilizer_map);
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
        assert_eq!("35", process(input)?);
        Ok(())
    }
}