#[warn(clippy::pedantic)]

fn main() {
    if 0.1 + 0.2 == 0.3 {
        println!("Math works");
    } else {
        println!("Oh dear");
    }

    if ((0.1 + 0.2) - 0.3f32).abs() < f32::EPSILON {
        println!("Approximate math still works");
    }
}
