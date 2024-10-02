use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Algorithm, Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier, Version,
};
use chrono::Utc;
use diesel::{insert_into, ExpressionMethods, QueryDsl, SelectableHelper};
use diesel_async::{pooled_connection::deadpool::Pool, AsyncPgConnection, RunQueryDsl};

use crate::{
    error::{DbError, ValidationError},
    models::{Birthday, User, UserToken},
    schema::{user_tokens, users},
    token::TokenGenerator,
    Result,
};

impl User {
    pub async fn create(
        username: impl AsRef<str>,
        email: impl AsRef<str>,
        password: impl AsRef<str>,
        birthday: &Birthday,
        pool: &Pool<AsyncPgConnection>,
    ) -> Result<User> {
        let user = User {
            birthday: birthday
                .as_timestamp()
                .ok_or(DbError::BirthdayConversionFailed)?
                .timestamp_millis(),
            created: Utc::now().timestamp_millis(),
            email: email.as_ref().to_string(),
            username: username.as_ref().to_string(),
            password: Self::hash_password(password)?,
            ..Default::default()
        };

        Ok(insert_into(users::table)
            .values(user)
            .returning(User::as_returning())
            .get_result(&mut pool.get().await?)
            .await?)
    }

    pub async fn login(
        username: impl AsRef<str>,
        password: impl AsRef<str>,
        pool: &Pool<AsyncPgConnection>,
    ) -> Result<UserToken> {
        let candidate: User = users::table
            .filter(users::username.eq(username.as_ref().to_string()))
            .select(User::as_select())
            .get_result(&mut pool.get().await?)
            .await?;

        if !Self::verify_password(password, candidate.password)? {
            return Err(ValidationError::IncorrectPassword.into());
        }

        let token = UserToken {
            value: TokenGenerator::generate_encoded_token(),
            created: Utc::now().timestamp_millis(),
            user_id: candidate.id.unwrap(),
            ..Default::default()
        };

        Ok(insert_into(user_tokens::table)
            .values(token)
            .returning(UserToken::as_returning())
            .get_result(&mut pool.get().await?)
            .await?)
    }

    pub fn alg() -> Result<Argon2<'static>> {
        Ok(Argon2::new(
            Algorithm::Argon2id,
            Version::V0x13,
            Params::new(
                // 24mb memory
                24 * 1024,
                // 2 iterations
                2,
                // 1 degrees of parallelism
                1,
                // Default output length
                None,
            )?,
        ))
    }

    pub fn hash_password(password: impl AsRef<str>) -> Result<String> {
        let salt = SaltString::generate(&mut OsRng);
        let hasher = Self::alg()?;

        Ok(hasher
            .hash_password(password.as_ref().as_bytes(), &salt)?
            .serialize()
            .to_string())
    }

    pub fn verify_password(input: impl AsRef<str>, real: impl AsRef<str>) -> Result<bool> {
        let hash = PasswordHash::new(real.as_ref())?;
        let algs: &[&dyn PasswordVerifier] = &[&Self::alg()?];

        Ok(hash.verify_password(algs, input.as_ref()).is_ok())
    }
}
