fn main() {
    let x: Option<u32> = None;
    let y = x.unwrap() + 4;
    println!("{y}");
}
