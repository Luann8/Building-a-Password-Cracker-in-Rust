use std::fs::File;
use std::io::{self, BufRead};
use ring::digest;

fn main() -> io::Result<()> {
    // Load the target password hash (e.g., SHA-256)
    let target_hash = "TARGET_HASH_HERE";

    // Load the wordlist file
    let wordlist_path = "WORDLIST_FILE_PATH_HERE";
    let file = File::open(wordlist_path)?;
    let reader = io::BufReader::new(file);

    // Iterate through each line in the wordlist
    for line in reader.lines() {
        let password = line?;

        // Hash the potential password using the same algorithm as the target hash
        let password_hash = hash_password(password.trim());

        // Compare the hashed potential password with the target hash
        if password_hash == target_hash {
            println!("Password cracked: {}", password);
            return Ok(());
        }
    }

    println!("Password not found in wordlist.");
    Ok(())
}

fn hash_password(password: &str) -> String {
    // Hash the password using a cryptographic hashing algorithm (e.g., SHA-256)
    let hashed_value = digest::digest(&digest::SHA256, password.as_bytes());

    // Convert the hashed value to a hexadecimal string
    let hex_string: String = hashed_value.as_ref().iter().map(|b| format!("{:02x}", b)).collect();
    hex_string
}
