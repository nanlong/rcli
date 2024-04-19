use rand::seq::SliceRandom;

const NUMBERS: &[u8] = b"123456789";
const UPPERS: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ";
const LOWERS: &[u8] = b"abcdefghijkmnopqrstuvwxyz";
const SYMBOLS: &[u8] = b"!@#$%^&*-_";

pub fn process_genpass(
    length: u8,
    no_upper: bool,
    no_lower: bool,
    no_num: bool,
    no_symbol: bool,
) -> anyhow::Result<String> {
    if no_upper && no_lower && no_num && no_symbol {
        return Err(anyhow::anyhow!(
            "At least one of no_upper, no_lower, no_num, no_symbol must be false"
        ));
    }

    let mut rng = rand::thread_rng();
    let mut chars = Vec::new();
    let mut password = Vec::new();

    if !no_upper {
        chars.extend_from_slice(UPPERS);
        password.push(*UPPERS.choose(&mut rng).expect("UPPERS won't be empty"));
    }
    if !no_lower {
        chars.extend_from_slice(LOWERS);
        password.push(*LOWERS.choose(&mut rng).expect("UPPERS won't be empty"));
    }
    if !no_num {
        chars.extend_from_slice(NUMBERS);
        password.push(*NUMBERS.choose(&mut rng).expect("NUMBERS won't be empty"));
    }
    if !no_symbol {
        chars.extend_from_slice(SYMBOLS);
        password.push(*SYMBOLS.choose(&mut rng).expect("SYMBOLS won't be empty"));
    }

    for _ in 0..length - password.len() as u8 {
        password.push(*chars.choose(&mut rng).expect("chars won't be empty"));
    }

    password.shuffle(&mut rng);
    Ok(String::from_utf8(password)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_genpass() {
        let password = process_genpass(16, false, false, false, false).unwrap();
        assert_eq!(password.len(), 16);
        assert!(
            has_upper_char(&password)
                && has_lower_char(&password)
                && has_num_char(&password)
                && has_symbol_char(&password)
        );
    }

    #[test]
    fn test_process_genpass_no_upper() {
        let password = process_genpass(16, true, false, false, false).unwrap();
        assert_eq!(password.len(), 16);
        assert!(!has_upper_char(&password));
        assert!(has_lower_char(&password) && has_num_char(&password) && has_symbol_char(&password));
    }

    #[test]
    fn test_process_genpass_no_lower() {
        let password = process_genpass(16, false, true, false, false).unwrap();
        assert_eq!(password.len(), 16);
        assert!(!has_lower_char(&password));
        assert!(has_upper_char(&password) && has_num_char(&password) && has_symbol_char(&password));
    }

    #[test]
    fn test_process_genpass_no_num() {
        let password = process_genpass(16, false, false, true, false).unwrap();
        assert_eq!(password.len(), 16);
        assert!(!has_num_char(&password));
        assert!(
            has_upper_char(&password) && has_lower_char(&password) && has_symbol_char(&password)
        );
    }

    #[test]
    fn test_process_genpass_no_symbol() {
        let password = process_genpass(16, false, false, false, true).unwrap();
        assert_eq!(password.len(), 16);
        assert!(!has_symbol_char(&password));
        assert!(has_upper_char(&password) && has_lower_char(&password) && has_num_char(&password));
    }

    fn has_upper_char(input: &str) -> bool {
        input.chars().any(|c| c.is_uppercase())
    }

    fn has_lower_char(input: &str) -> bool {
        input.chars().any(|c| c.is_lowercase())
    }

    fn has_num_char(input: &str) -> bool {
        input.chars().any(|c| c.is_numeric())
    }

    fn has_symbol_char(input: &str) -> bool {
        input.chars().any(|c| SYMBOLS.contains(&(c as u8)))
    }
}
