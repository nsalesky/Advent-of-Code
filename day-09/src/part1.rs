use anyhow::{anyhow, Result};

fn predict_next_element(sequence: &Vec<i32>) -> Result<i32> {
    if sequence.len() < 2 {
        return Err(anyhow!("not enough values to compute a difference"));
    }

    let mut differences = Vec::with_capacity(sequence.len() - 1);
    for (i, value) in sequence.iter().enumerate().skip(1) {
        differences.push(value - sequence[i - 1]);
    }

    if differences.iter().all(|v| v == &0) {
        Ok(sequence[0])
    } else {
        let next_difference = predict_next_element(&differences)?;
        Ok(sequence.last().expect("differences is non-empty") + next_difference)
    }
}

fn process_line(line: &str) -> Result<i32> {
    let values: Vec<i32> = line.split(' ')
        .map(|v| v.parse().expect("value is an integer"))
        .collect();

    predict_next_element(&values)
}

pub fn process(input: &str) -> Result<String> {
    let mut result = 0;
    for line in input.lines() {
        result += process_line(line)?;
    }
    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> Result<()> {
        let input = "\
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        assert_eq!("114", process(input)?);
        Ok(())
    }

    #[test]
    fn test_process_line() -> Result<()> {
        assert_eq!(18, process_line("0 3 6 9 12 15")?);
        Ok(())
    }
}