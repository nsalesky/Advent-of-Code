use std::cmp::min;
use std::collections::LinkedList;
use std::ops::Range;
use anyhow::Result;
use itertools::Itertools;
use rangemap::{RangeMap, RangeSet};

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

fn process_ranges<T>(ranges: T, source_to_dest_map: &RangeMap<u64, u64>) -> Vec<Range<u64>>
where
T: IntoIterator<Item = Range<u64>>
{
    let mut results_set = RangeSet::new();

    let mut worklist = LinkedList::from_iter(ranges);

    while let Some(item_range) = worklist.pop_front() {
        let first_item = item_range.start;
        let last_item = item_range.end;
        match source_to_dest_map.get_key_value(&first_item) {
            Some((source_range, dest_start)) => {
                let num_elements = min(
                    last_item - first_item,
                    last_item - source_range.start,
                );
                let num_remaining_elements = last_item - (first_item + num_elements);

                let dest_range = *dest_start..(*dest_start + num_elements);

                results_set.insert(dest_range);

                if num_remaining_elements > 0 {
                    worklist.push_back(first_item+num_elements..last_item);
                }
            },
            None => {
                results_set.insert(first_item..first_item+1);

                if first_item + 1 < last_item {
                    worklist.push_back(first_item+1..last_item);
                }
            }
        };
    }

    results_set
        .iter()
        .map(|range| range.clone())
        .collect()
}

pub fn process(input: &str) -> Result<String> {
    let sections: Vec<&str> = input.split("\n\n").collect();
    assert_eq!(sections.len(), 8);

    let starting_seed_ranges: Vec<Range<u64>> = sections[0]
        .strip_prefix("seeds: ")
        .expect("first line lists the seeds")
        .split(' ')
        .map(|seed| seed.parse::<u64>().expect("seed part is numeric"))
        .tuples::<(u64, u64)>()
        .map(|(start, length)| start..start+length)
        .collect();


    let seed_to_soil_map = parse_section_ranges(sections[1]);
    let soil_to_fertilizer_map = parse_section_ranges(sections[2]);
    let fertilizer_to_water_map = parse_section_ranges(sections[3]);
    let water_to_light_map = parse_section_ranges(sections[4]);
    let light_to_temperature_map = parse_section_ranges(sections[5]);
    let temperature_to_humidity_map = parse_section_ranges(sections[6]);
    let humidity_to_location_map = parse_section_ranges(sections[7]);

    let soil_ranges = process_ranges(starting_seed_ranges, &seed_to_soil_map);
    let fertilizer_ranges = process_ranges(soil_ranges, &soil_to_fertilizer_map);
    let water_ranges = process_ranges(fertilizer_ranges, &fertilizer_to_water_map);
    let light_ranges = process_ranges(water_ranges, &water_to_light_map);
    let temperature_ranges = process_ranges(light_ranges, &light_to_temperature_map);
    let humidity_ranges = process_ranges(temperature_ranges, &temperature_to_humidity_map);
    let location_ranges = process_ranges(humidity_ranges, &humidity_to_location_map);

    let min_location = location_ranges
        .iter()
        .map(|range| range.start)
        .reduce(min)
        .expect("there is at least one location");

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
