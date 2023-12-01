mod day1;

use anyhow::Result;

fn main() -> Result<()> {
    let result = day1::part2(include_str!("day1/input1.txt"))?;
    println!("result was: {}", result);
    Ok(())
}
