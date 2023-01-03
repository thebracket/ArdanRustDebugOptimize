use anyhow::Result;

fn requires_a_positive_int<T>(number: T) -> Result<u32, T::Error> 
where T: TryInto<u32>
{
    let number: u32 = number.try_into()?;
    Ok(number * 3)
}

fn requires_a_float<T>(number: T) -> Result<f64, T::Error> 
where T: TryInto<f64>
{
    Ok(number.try_into()? * 2.0)
}

fn main() -> Result<()> {
    let n:i32 = 5;
    println!("{}", requires_a_positive_int(n)?);
    println!("{}", requires_a_float(n)?);
    Ok(())
}
