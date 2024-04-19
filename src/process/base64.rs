use crate::Base64Format;
use base64::prelude::*;
use std::io::Read;

pub enum Base64Action {
    Decode,
    Encode,
}

pub fn process_base64(
    input: &mut dyn Read,
    format: &Base64Format,
    action: Base64Action,
) -> anyhow::Result<String> {
    let mut buf = String::new();
    input.read_to_string(&mut buf)?;
    let buf = buf.trim();

    let result = match (format, action) {
        (Base64Format::Standard, Base64Action::Encode) => BASE64_STANDARD.encode(buf),
        (Base64Format::Standard, Base64Action::Decode) => {
            String::from_utf8(BASE64_STANDARD.decode(buf)?)?
        }

        (Base64Format::UrlSafe, Base64Action::Encode) => BASE64_URL_SAFE_NO_PAD.encode(buf),
        (Base64Format::UrlSafe, Base64Action::Decode) => {
            String::from_utf8(BASE64_URL_SAFE_NO_PAD.decode(buf)?)?
        }
    };

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_base64_encode_standard() {
        let input = "hello world";
        let mut input = input.as_bytes();
        let result =
            process_base64(&mut input, &Base64Format::Standard, Base64Action::Encode).unwrap();
        assert_eq!(result, "aGVsbG8gd29ybGQ=");
    }

    #[test]
    fn test_process_base64_decode_standard() {
        let input = "aGVsbG8gd29ybGQ=";
        let mut input = input.as_bytes();
        let result =
            process_base64(&mut input, &Base64Format::Standard, Base64Action::Decode).unwrap();
        assert_eq!(result, "hello world");
    }

    #[test]
    fn test_process_base64_encode_urlsafe() {
        let input = "hello world";
        let mut input = input.as_bytes();
        let result =
            process_base64(&mut input, &Base64Format::UrlSafe, Base64Action::Encode).unwrap();
        assert_eq!(result, "aGVsbG8gd29ybGQ");
    }

    #[test]
    fn test_process_base64_decode_urlsafe() {
        let input = "aGVsbG8gd29ybGQ";
        let mut input = input.as_bytes();
        let result =
            process_base64(&mut input, &Base64Format::UrlSafe, Base64Action::Decode).unwrap();
        assert_eq!(result, "hello world");
    }
}
