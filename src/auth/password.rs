use argon2::{
    Argon2, PasswordVerifier, password_hash::{PasswordHasher, phc::{PasswordHash, Salt}}
};


pub fn hash_password(plain: &str) -> Result<String, String> {
    let salt = Salt::generate();
    let argon2 = Argon2::default();

    argon2
        .hash_password_with_salt(plain.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|err| err.to_string())

}

pub fn verify_password(plain: &str, hash: &str) -> bool {
    let parsed_hash = match PasswordHash::new(hash) {
        Ok(h) => h,
        Err(_) => return false,
    };
    Argon2::default()
        .verify_password(plain.as_bytes(), &parsed_hash)
        .is_ok()
}