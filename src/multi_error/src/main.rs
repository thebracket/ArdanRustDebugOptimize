use std::{error::{self, Error}, fmt::{Display, self}};

#[derive(Debug)]
enum CalculationError { NotEqualToFour }
impl Error for CalculationError {}
impl Display for CalculationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There is an error: {:?}", self)
    }
}

#[derive(Debug)]
enum MemoryError {OutOfMemory}
impl Error for MemoryError {}
impl Display for MemoryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There is an error: {:?}", self)
    }
}

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

fn big_calculation(n: u32) -> Result<u32> {
    match n {
        4 => Ok(3),
        5.. => Err(Box::<MemoryError>::new(MemoryError::OutOfMemory.into())),
        _ => Err(Box::<CalculationError>::new(CalculationError::NotEqualToFour.into())),
    }
}

fn main() {
    for i in 0..6 {
        println!("{:?}", big_calculation(i));
    }
}
