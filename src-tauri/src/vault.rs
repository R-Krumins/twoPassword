use std::fs;

use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm,
};
use sha2::{Digest, Sha256};

pub struct Vault {
    path: String,
    cipher: Aes256Gcm,
}

impl Vault {
    pub fn new(path: String, key: String) -> Vault {
        let mut hasher = Sha256::new();
        hasher.update(key.as_bytes());
        let key = hasher.finalize();

        Vault {
            path,
            cipher: Aes256Gcm::new(&key),
        }
    }

    pub fn write(&self, data: String) -> Result<(), String> {
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
        let ciphertext = self
            .cipher
            .encrypt(&nonce, data.as_bytes())
            .map_err(|_| "Failed to encrypt data".to_string())?;

        let mut file_data = nonce.to_vec();
        file_data.extend_from_slice(&ciphertext);

        fs::write(&self.path, file_data).map_err(|e| e.to_string())?;

        Ok(())
    }

    pub fn read(&self) -> Result<String, String> {
        let file_data = fs::read(&self.path).map_err(|e| e.to_string())?;

        // nonce is 12 bytes
        if file_data.len() < 12 {
            return Err("Failed to decrypt: file less than 12 bytes".to_string());
        }

        let (nonce_bytes, ciphertext) = file_data.split_at(12);
        let nonce = aes_gcm::Nonce::from_slice(nonce_bytes);

        let plaintext = self
            .cipher
            .decrypt(nonce, ciphertext)
            .map_err(|_| "Failed to decrypt data: something went horibbly wrong".to_string())?;

        String::from_utf8(plaintext)
            .map_err(|_| "Failed to decrypt: decrypted data is not valid UTF-8".to_string())
    }
}
