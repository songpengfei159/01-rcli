use rand::Rng;
const UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
const NUMBERS: &str = "0123456789";
const SYMBOLS: &str = "!@#$%^&*()-_=+";
pub fn generate_password(
    length: u8,
    uppercase: bool,
    numbers: bool,
    lowercase: bool,
    symbols: bool,
) -> anyhow::Result<String> {
    let mut password = String::new();
    let mut rng = rand::thread_rng();
    let mut char_set = String::new();
    if uppercase {
        char_set.push_str(UPPERCASE);
    }
    if lowercase {
        char_set.push_str(LOWERCASE);
    }
    if numbers {
        char_set.push_str(NUMBERS);
    }
    if symbols {
        char_set.push_str(SYMBOLS);
    }
    for _ in 0..length {
        let idx = rng.gen_range(0..char_set.len());
        password.push(char_set.chars().nth(idx).unwrap());
    }
    println!("{}", password);
    Ok(password)
}
