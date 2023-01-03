use anyhow::{Result, Error};

fn big_calculation(n: u32) -> Result<u32> {
    match n {
        4 => Ok(3),
        5.. => Err(Error::msg("Out of memory")),
        _ => Err(Error::msg("Not equal to four")),
    }
}

fn main() {
    for i in 0..6 {
        println!("{:?}", big_calculation(i));
    }
}
