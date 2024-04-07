## Password Cracking Tool Instructions

### Overview:
This Rust program attempts to crack a password hash using a wordlist. It hashes each word from the wordlist using SHA-256 and compares the hashed value with a target hash. If a match is found, it prints the cracked password.

### Instructions:
1. **Prepare Target Hash:**
   - Replace `"TARGET_HASH_HERE"` with the hash of the password you want to crack. Ensure it's in hexadecimal format and matches the hash algorithm used in the code (SHA-256).

2. **Prepare Wordlist:**
   - Replace `"WORDLIST_FILE_PATH_HERE"` with the path to your wordlist file. Each line in the file should contain a single word.

3. **Setup Rust Environment:**
   - Ensure you have Rust installed on your system. If not, you can download and install it from [Rust's official website](https://www.rust-lang.org/tools/install).

4. **Compile and Run:**
   - Copy the provided code into a Rust source file (e.g., `main.rs`).
   - Open a terminal or command prompt and navigate to the directory containing your Rust source file.
   - Run the following command to compile and execute the program:
     ```bash
     cargo run
     ```
   - The program will search through the wordlist and attempt to crack the password hash. If successful, it will print the cracked password.

5. **Adjustments (Optional):**
   - You can modify the code to use a different hashing algorithm or adjust the hashing process according to your requirements.

6. **Wordlist Considerations:**
   - Ensure that your wordlist contains a variety of words and is suitable for the target password. A comprehensive and diverse wordlist improves the chances of success.

7. **Caution:**
   - Use this tool responsibly and only for legitimate purposes. Unauthorized use may violate privacy and security laws.

### Notes:
- This program is for educational purposes and should not be used for illegal activities.
- Ensure you have permission to crack the password you are attempting to guess.
