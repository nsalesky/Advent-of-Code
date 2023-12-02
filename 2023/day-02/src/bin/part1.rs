use anyhow::Result;
use day_02::part1::process;

fn main() -> Result<()> {
    let file = include_str!("../../input1.txt");
    let result = process(file)?;
    println!("{}", result);
    Ok(())
}