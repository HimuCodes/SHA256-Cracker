**SHA256 Cracker**

A tool I made to crack SHA256 hashes with a password list in Rust. 
This helped me to learn the basics of borrowing in rust as well as converting Strings to byte code and also how to use external crates.
This is a simple little side project I did instead of studying for my exams. ;)

**Code Structure**

The code utilizes the following libraries:

- `std::env`: Retrieves arguments passed to the program.
- `std::fs`: Manages file operations.
- `std::io`: Provides input/output facilities.
- `sha2`: Implements the SHA-256 hashing algorithm.
- `std::process`: Allows process termination.

**Explanation**

1. **Command-Line Arguments:** The code expects a single argument: the SHA-256 hash value you want to compare against.
2. **Error Handling:** If an invalid number of arguments is provided, an error message is displayed, and the program exits.
3. **Hash Retrieval:** The desired hash is stored in a variable.
4. **Password File:** A static path to a password file (replace with your own or remove this section if not used) is defined.
5. **Attempt Counter:** A variable to track the number of password attempts is initialized.
6. **Hash Comparison Loop:**
   - Each line in the password file is read.
   - The password is trimmed and converted to a string.
   - The password is hashed using SHA-256.
   - The attempt number, password, and hashed value are printed for informational purposes (remove if not desired).
   - If the password's hash matches the desired hash, the program exits with a success message.
7. **Failure Message:** If the loop completes without finding a match, a message indicating the hash wasn't found is printed.

**Running the Code**

```bash
cargo run <SHA-256 hash>
```

**Important Note**

- Replace `<SHA-256 hash>` with the actual SHA-256 hash value you want to verify.

