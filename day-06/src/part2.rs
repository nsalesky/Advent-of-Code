use anyhow::Result;
use regex::Regex;

fn number_of_ways_to_win(time_limit: u64, record_distance: u64) -> u32 {
    (0..=time_limit)
        .into_iter()
        .map(|charge_up_time| {
            let speed = charge_up_time; // just making this distinction clear
            speed * (time_limit - charge_up_time)
        })
        .filter(|distance| distance > &record_distance)
        .count() as u32
}

pub fn process(input: &str) -> Result<String> {
    let numbers_regex = Regex::new(r"\d+")?;

    let lines: Vec<&str> = input.lines().collect();
    assert_eq!(lines.len(), 2, "there are more than two lines in the input");

    let time_limit = numbers_regex
        .find_iter(lines[0])
        .map(|time_limit| time_limit.as_str())
        .collect::<String>()
        .parse::<u64>()
        .expect("time limit is an integer");
    let record_distance = numbers_regex
        .find_iter(lines[1])
        .map(|record| record.as_str())
        .collect::<String>()
        .parse::<u64>()
        .expect("record distance is an integer");

    let result = number_of_ways_to_win(time_limit, record_distance);
    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn test_process() -> Result<()> {
        let input = "\
Time:      7  15   30
Distance:  9  40  200";
        assert_eq!("71503", process(input)?);
        Ok(())
    }

    #[rstest]
    #[case(7, 9, 4)]
    #[case(15, 40, 8)]
    #[case(30, 200, 9)]
    fn test_number_of_ways_to_win(#[case] time_limit: u64, #[case] previous_record: u64, #[case] expected: u32) {
        assert_eq!(number_of_ways_to_win(time_limit, previous_record), expected);
    }
}
