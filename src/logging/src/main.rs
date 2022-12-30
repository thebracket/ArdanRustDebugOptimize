use std::fmt::{Display, Formatter};

#[derive(Debug)]
struct User {
    name: String,
    full_name: String,
    age: Age,
}

impl Display for User {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{} ({})", self.full_name, self.age)
    }
}

#[derive(Debug)]
struct Age {
    birth_year: u32,
}

impl Display for Age {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", 2023 - self.birth_year)
    }
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
    env_logger::init();

    let users = vec![
        User{
            name: "Herbert".to_string(),
            full_name: "Herbert Wolverson".to_string(),
            age: Age{birth_year: 1975},
        }
    ];

    println!("Hello, what's your name?");
    let name = read_line()
        .expect("Unable to read from stdin");

    let user = users.iter().find(|u| clean_string(&u.name) == name);
    match user {
        Some(user) => log::info!("{user} logged in"),
        None => log::error!("Invalid user: {}", name),
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