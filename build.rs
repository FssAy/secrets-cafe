use std::fs::OpenOptions;
use std::io::Write;
use rand::RngCore;
use rand::rngs::OsRng;

const HMAC_KEY_PATH: &str = "src/secrets/token-hmac-key.bin";

fn main() {
    // forces build.rs to run every time.
    println!("cargo:rerun-if-changed=NULL");

    OpenOptions::new()
        .write(true)
        .append(false)
        .truncate(true)
        .create(true)
        .open(HMAC_KEY_PATH)
        .expect("Failed to open the HMAC key file!")
        .write_all(&create_new_hmac_key())
        .expect("Failed to write data to the HMAC key file!");
}

fn create_new_hmac_key() -> [u8; 32] {
    let mut buffer = [0u8; 32];
    OsRng.fill_bytes(&mut buffer);
    buffer
}
