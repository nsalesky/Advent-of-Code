use day_01::part1::process;
use anyhow::Result;

fn main() -> Result<()> {
    let file = include_str!("../../input1.txt");
    let result = process(file)?;
    println!("{}", result);
    Ok(())
}