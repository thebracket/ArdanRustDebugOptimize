fn main() {
    let n = u64::MAX;
    let n:u32 = n.try_into().unwrap_or(0);
    println!("{n}");

    let n:i32 = 500;
    let n:u32 = n.try_into().unwrap_or(0);
    println!("{n}");
}
