fn main() {
    println!("Hello, what's your name?");
    let mut buffer = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut buffer).expect(
        "Unable to read standard input"
    );

    if buffer == "Herbert" {
        println!("Hello Herbert");
    } else {
        println!("You aren't an authorized user.")
    }
}
