use rand::rngs::OsRng;
use rsa::pkcs1::{EncodeRsaPrivateKey, EncodeRsaPublicKey};
use rsa::pkcs8::LineEnding;
use rsa::{RsaPrivateKey, RsaPublicKey};
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

fn main() {
    fs::create_dir_all("./malcrypt").expect("Failed to create sample compilation path");

    // Generate RSA key pair
    let mut rng = OsRng;
    let bits = 2048;
    let private_key = RsaPrivateKey::new(&mut rng, bits).expect("Failed to generate a key");
    let public_key = RsaPublicKey::from(&private_key);

    // Save private key to a file
    let private_key_pem = private_key
        .to_pkcs1_pem(LineEnding::default())
        .expect("Failed to convert private key to PEM");

    let private_key_path = Path::new("malcrypt/prvkey.pem");
    let mut private_key_file =
        File::create(private_key_path).expect("Failed to create private key file");

    private_key_file
        .write_all(private_key_pem.as_bytes())
        .expect("Failed to write private key");

    // Save public key to a file
    let public_key_pem = public_key
        .to_pkcs1_pem(LineEnding::default())
        .expect("Failed to convert public key to PEM");

    let public_key_path = Path::new("malcrypt/pubkey.pem");
    let mut public_key_file =
        File::create(public_key_path).expect("Failed to create public key file");

    public_key_file
        .write_all(public_key_pem.as_bytes())
        .expect("Failed to write public key");

    println!("cargo:rerun-if-changed=build.rs");
}
