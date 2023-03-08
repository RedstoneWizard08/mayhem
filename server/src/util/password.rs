use pbkdf2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Pbkdf2,
};

use rand::rngs::OsRng;

pub fn hash(input: &str) -> String {
    let password = input.as_bytes();
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = Pbkdf2.hash_password(password, &salt).unwrap().to_string();
    let parsed_hash = PasswordHash::new(&password_hash).unwrap();

    assert!(Pbkdf2.verify_password(password, &parsed_hash).is_ok());

    return password_hash;
}
