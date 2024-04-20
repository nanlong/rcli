use std::{
    str::FromStr,
    time::{SystemTime, UNIX_EPOCH},
};

use anyhow::Result;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    aud: String, // Optional. Audience
    exp: usize, // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    iat: usize, // Optional. Issued at (as UTC timestamp)
    iss: String, // Optional. Issuer
    nbf: usize, // Optional. Not Before (as UTC timestamp)
    sub: String, // Optional. Subject (whom token refers to)
}

impl Claims {
    pub fn try_new(aud: &str, exp: &str, iss: &str, sub: &str) -> Result<Self> {
        let now = now();

        Ok(Self {
            aud: aud.to_string(),
            exp: exp.parse::<TimeDelta>()?.as_timestamp(),
            iat: now,
            iss: iss.to_string(),
            nbf: now,
            sub: sub.to_string(),
        })
    }
}

fn now() -> usize {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    since_the_epoch.as_secs() as usize
}

pub enum TimeDelta {
    Hours(usize),
    Days(usize),
    Weeks(usize),
    Months(usize),
}

impl TimeDelta {
    pub fn as_timestamp(&self) -> usize {
        let now = now();

        match self {
            Self::Hours(h) => now + h * 3600,
            Self::Days(d) => now + d * 86400,
            Self::Weeks(w) => now + w * 604800,
            Self::Months(m) => now + m * 2592000,
        }
    }
}

impl FromStr for TimeDelta {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (num, unit) = s.split_at(s.len() - 1);
        let num = num.parse()?;
        match unit {
            "h" => Ok(Self::Hours(num)),
            "d" => Ok(Self::Days(num)),
            "w" => Ok(Self::Weeks(num)),
            "m" => Ok(Self::Months(num)),
            _ => Err(anyhow::anyhow!("Invalid time unit")),
        }
    }
}

pub fn process_jwt_sign(key: &str, aud: &str, exp: &str, iss: &str, sub: &str) -> Result<String> {
    let key = EncodingKey::from_secret(key.as_ref());
    let claims = Claims::try_new(aud, exp, iss, sub)?;
    let token = encode(&Header::default(), &claims, &key)?;

    Ok(token)
}

pub fn process_jwt_verify(key: &str, token: &str, aud: &str) -> Result<()> {
    let key = DecodingKey::from_secret(key.as_ref());
    let mut validation = Validation::default();

    validation.set_audience(&[aud]);
    decode::<Claims>(token, &key, &validation)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_delta() {
        let td = "1h".parse::<TimeDelta>().unwrap();
        assert_eq!(td.as_timestamp(), now() + 3600);

        let td = "1d".parse::<TimeDelta>().unwrap();
        assert_eq!(td.as_timestamp(), now() + 86400);

        let td = "1w".parse::<TimeDelta>().unwrap();
        assert_eq!(td.as_timestamp(), now() + 604800);

        let td = "1m".parse::<TimeDelta>().unwrap();
        assert_eq!(td.as_timestamp(), now() + 2592000);
    }

    #[test]
    fn test_process_sign() {
        let key = "secret";
        let aud = "aud";
        let exp = "1h";
        let iss = "iss";
        let sub = "sub";

        let token = process_jwt_sign(key, aud, exp, iss, sub).unwrap();
        let verified = process_jwt_verify(key, &token, aud);
        assert!(verified.is_ok());
    }
}
