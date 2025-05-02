use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng}, Aes256Gcm, Error, Key, Nonce // Or `Aes128Gcm`
};
use std::io;

pub fn aes_encrypt_with_key(key: [u8; 32], chunk: &[u8]) -> Result<Vec<u8>, Error> {

    //generates an output that contains the data + the nonce at the end
    let key = Key::<Aes256Gcm>::from_slice(&key);
    let cipher = Aes256Gcm::new(key);

    let nonce = Aes256Gcm::generate_nonce(&mut OsRng); // 96-bit nonce
    let ciphertext = cipher.encrypt(&nonce, chunk)?;

    // Prepend the nonce to the ciphertext
    let mut output = Vec::with_capacity(nonce.len() + ciphertext.len());
    output.extend_from_slice(nonce.as_slice());
    output.extend_from_slice(&ciphertext);

    Ok(output)
}
pub fn aes_decrypt_with_key(key: [u8; 32], chunk: &[u8]) -> Result<Vec<u8> , Error> {
    let key = Key::<Aes256Gcm>::from_slice(&key);
    let cipher = Aes256Gcm::new(key);

    let (nonce_bytes, ciphertext) = chunk.split_at(12); // 96 bits
    let nonce = Nonce::from_slice(nonce_bytes);

    cipher.decrypt(nonce, ciphertext)
}


pub fn password_to_key<const N: usize>(password: &str) -> io::Result<[u8; N]> {
    let mut buf = [0u8; N];
    let bytes = password.as_bytes();

    if bytes.len() > N {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Password too long"));
    }

    buf[..bytes.len()].copy_from_slice(bytes);
    Ok(buf)
}

pub fn password_to_key32(password: &str) -> io::Result<[u8; 32]> {
    password_to_key::<32>(password)
}



