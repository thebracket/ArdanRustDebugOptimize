struct User {
    name: String,
    full_name: String,
    age: u32,
}

fn clean_string(s: &String) -> String {
    s.trim().to_lowercase()
}

fn read_line() -> std::io::Result<String> {
    let mut buffer = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut buffer)?;
    Ok(clean_string(&buffer))
}

fn main() {
    let users = vec![
        User{
            name: "Herbert".to_string(),
            full_name: "Herbert Wolverson".to_string(),
            age: 47,
        }
    ];

    println!("Hello, what's your name?");
    let name = read_line()
        .expect("Unable to read from stdin");

    let user = users.iter().find(|u| clean_string(&u.name) == name);
    match user {
        Some(user) => {
            println!("Full Name: {}", user.full_name);
            println!("Age: {}", user.age);
        },
        None => println!("I've no idea who you are!"),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_unix_input() {
        assert_eq!(clean_string(&"herbert\n".to_string()), "herbert");
    }

    #[test]
    fn test_windows_input() {
        assert_eq!(clean_string(&"herbert\r\n".to_string()), "herbert");
    }

    #[test]
    fn test_overly_tabby() {
        assert_eq!(clean_string(&"\t\therbert\n".to_string()), "herbert");
    }

    #[test]
    fn test_too_much_coffee() {
        assert_eq!(clean_string(&"HeRbErT".to_string()), "herbert");
    }
}