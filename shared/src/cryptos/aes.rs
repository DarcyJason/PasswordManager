use aes_gcm::{
    AeadCore, Aes256Gcm, Key, KeyInit,
    aead::{Aead, OsRng, consts::U12, generic_array::GenericArray},
};

use crate::result::SharedLibResult;

pub fn generate_nonce() -> GenericArray<u8, U12> {
    Aes256Gcm::generate_nonce(&mut OsRng)
}

pub fn encrypted(
    key: &[u8; 32],
    nonce: GenericArray<u8, U12>,
    plaintext: String,
) -> SharedLibResult<String> {
    let key: &Key<Aes256Gcm> = key.into();
    let cipher = Aes256Gcm::new(&key);
    let cipher_vec = cipher.encrypt(&nonce, plaintext.as_bytes().as_ref())?;
    Ok(String::from_utf8(cipher_vec)?)
}

pub fn decrypted(
    key: &[u8; 32],
    nonce: GenericArray<u8, U12>,
    ciphertext: String,
) -> SharedLibResult<String> {
    let key: &Key<Aes256Gcm> = key.into();
    let cipher = Aes256Gcm::new(&key);
    let plain_vec = cipher.decrypt(&nonce, ciphertext.as_bytes().as_ref())?;
    Ok(String::from_utf8(plain_vec)?)
}
