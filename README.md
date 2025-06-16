![true meme](https://i.imgflip.com/9xio6d.jpg)

# 🔐 2Password

2Password is a password manager written in Tauri, using Svelte for the frontend and Rust for the backend.

2Password creates and interfaces with encrypted files that store your logins. Provide a file path and a master password and 2Password will handle the rest.

> [!CAUTION]
> This was made for learning purposes, don't actually use it to store your passwords, you absolute buffoon.

### Crypto

The Rust backend uses the Sha2 and AES_GCM libraries for the cryptography.
The encryption process is as folllows:

1. The user's password is hashed using SHA256
2. A random nonce is generated
3. The plaintext json is encrypted using AES256 GCM
4. The nonce is prepended to the cyphertext

### Why Tauri?

Tauri was selected primarily for its focus on security, small binary size, and seamless integration of a Rust backend with modern web technologies.

# 🛠 How to build?

Requires: `rust` and `npm` to be installed.

```bash
cd twoPassword
npm i
npm run tauri dev
```
