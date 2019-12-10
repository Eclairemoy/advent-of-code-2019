use std::io::prelude::*;
use ferris_says::say;
use std::io::{stdout, BufWriter, BufReader};
use std::fs::File;

fn main() -> std::io::Result<()> {
    let f = File::open("src/input.txt")?;
    let mut reader = BufReader::new(f);
    let mut cumulative_fuel_count: u32 = 0;

    for line in reader.lines() {
        let mass: u32 = line?.parse::<u32>().unwrap();
        cumulative_fuel_count += fuel_required(mass);
    }
    
    println!("{}", cumulative_fuel_count.to_string());
    
    Ok(())
}

fn fuel_required(mass: u32) -> u32 {
    return (mass / 3) - 2;
}
