use anyhow::{Result, Error};

fn big_task() -> Result<()> {
    // () lets you return success without a value
    Ok(())
}

fn small_task() -> Result<()> {
    Err(Error::msg("We ran out of disk space"))
}

fn main() -> Result<()> {
    big_task()?;
    small_task()?;
    Ok(())
}
