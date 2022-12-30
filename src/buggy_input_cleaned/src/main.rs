fn clean_string(s: String) -> String {
    s.trim().to_lowercase()
}

fn read_line() -> std::io::Result<String> {
    let mut buffer = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut buffer)?;
    Ok(clean_string(buffer))
}

fn main() {
    println!("Hello, what's your name?");
    let name = read_line()
        .expect("Unable to read from stdin");

    if name == "herbert" {
        println!("Hello Herbert!");
    } else {
        println!("I've no idea who you are!");
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_unix_input() {
        assert_eq!(clean_string("herbert\n".to_string()), "herbert");
    }

    #[test]
    fn test_windows_input() {
        assert_eq!(clean_string("herbert\r\n".to_string()), "herbert");
    }

    #[test]
    fn test_overly_tabby() {
        assert_eq!(clean_string("\t\therbert\n".to_string()), "herbert");
    }

    #[test]
    fn test_too_much_coffee() {
        assert_eq!(clean_string("HeRbErT".to_string()), "herbert");
    }
}