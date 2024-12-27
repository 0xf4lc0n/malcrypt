# MALcrypt

MALcrypt is a proof-of-concept (PoC) ransomware developed primarily by ChatGPT, with minor code modifications to ensure compilation and execution. During the build phase, a pair of RSA keys is generated. The public key is embedded into the MALcrypt binary and is utilized for key encryption.

**How MALcrypt Works:**

1. **RSA Key Generation:**
   - MALcrypt generates a random RSA key pair.

2. **RSA Key Encryption:**
   - The private RSA key is encrypted with a hardcoded RSA public key.

3. **RC4 Key Generation:**
   - For each file, MALcrypt generates a unique RC4 key.

4. **File Encryption:**
   - MALcrypt traverses the file system and encrypts each eligible file using the RC4 algorithm.
   - The encrypted RC4 key, along with the encrypted RSA key from step 1, is appended to each encrypted file.

5. **Saving Encrypted RSA Key:**
   - The encrypted RSA key obtained in step 2 is saved on the disk.

6. **Encryption Note:**
   - MALcrypt generates an encryption note.

**Build Requirement:**

When building MALcrypt, the definition of the `PATH_TO_ENCRYPT` variable is required. This variable specifies the root directory from which the encryption process begins.

**Purpose:**

MALcrypt was built specifically for performing antivirus (AV) evasion tests.

**Caution:**

Users should exercise extreme caution when executing MALcrypt, as it has the capability to encrypt the entire machine and render data inaccessible.

---
