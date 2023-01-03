fn maybe_success(n: u32) -> Option<u32> {
    if n == 4 {
        Some(3)
    } else {
        None
    }
}

#[derive(Debug)]
enum CalculationError {
    NotEqualToFour
}

fn success_or_error(n: u32) -> Result<u32, CalculationError> {
    if n == 4 {
        Ok(3)
    } else {
        Err(CalculationError::NotEqualToFour)
    }
}

fn main() {
    const N:u32 = 4;
    match maybe_success(N) {
        Some(result) => println!("Maybe Success returned: {result}"),
        None => println!("Maybe Success return None"),
    }

    match success_or_error(N) {
        Ok(result) => println!("Success or Error returned {result}"),
        Err(e) => println!("Error: {:?}", e),
    }
}
