fn should_be_four(n: u32) {
    if n != 4 {
        panic!("n should be equal to 4");
    }
}

fn main() {
    // Do some long, hard math
    let n = 5;
    
    // Send to a critical function that expects n = 4
    should_be_four(n);
}
