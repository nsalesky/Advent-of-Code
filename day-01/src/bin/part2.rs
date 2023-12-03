use day_01::part2::process;
use anyhow::Result;

fn main() -> Result<()> {
    let file = include_str!("../../input2.txt");
    let result = process(file)?;
    println!("{}", result);
    Ok(())
}
