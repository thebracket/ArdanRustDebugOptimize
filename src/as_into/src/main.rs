fn main() {
    let n = u64::MAX;
    let n:u32 = n.into();
    println!("{n}");

    let n:i32 = -500;
    let n:i32 = n.into();
    println!("{n}");
}
