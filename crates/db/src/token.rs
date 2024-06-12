use base64::{engine::general_purpose, Engine};

use crate::Result;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct TokenGenerator;

impl TokenGenerator {
    /// Token version 1.0.0 as of 6/12/2024
    /// Token standard: "mhm$$[version]$$[token]"
    pub const TOKEN_VERSION: &'static str = "v1.0.0";

    /// Token length is 64 characters + the prefix.
    pub const TOKEN_LENGTH: usize = 64;

    /// The available characters are the upper and
    /// lowercase alphabets, and number 0 through 9.
    pub const TOKEN_CHARSET: &'static str =
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

    /// The time until a token expires in milliseconds.
    /// Calculation: 1 * SECS_PER_WEEK * MILLIS_PER_SEC
    /// See https://docs.rs/chrono/latest/src/chrono/duration.rs.html#39
    /// and https://docs.rs/chrono/latest/src/chrono/duration.rs.html#31
    #[allow(clippy::identity_op)]
    pub const TOKEN_EXPIRE_TIME: i64 = 1 * 604800 * 1000;

    fn generate_raw_token() -> String {
        format!(
            "mhm$${}$${}",
            Self::TOKEN_VERSION,
            random_string::generate(Self::TOKEN_LENGTH, Self::TOKEN_CHARSET)
        )
    }

    pub fn generate_encoded_token() -> String {
        let token = Self::generate_raw_token();

        general_purpose::STANDARD_NO_PAD.encode(token.as_bytes())
    }

    pub fn decode_token(token: impl AsRef<str>) -> Result<(String, String)> {
        let raw = String::from_utf8(general_purpose::STANDARD_NO_PAD.decode(token.as_ref())?)?;
        let parts = raw.split("$$").collect::<Vec<_>>();

        Ok((parts[1].into(), parts[2].into()))
    }
}
