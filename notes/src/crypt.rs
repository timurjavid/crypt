use aes::Aes256;
use block_modes::Cbc;
use block_modes::{BlockMode, BlockModeError};
use rand::Rng;
use block_modes::block_padding::Pkcs7;
use sha2::{Sha256, Digest};

type Aes256Cbc = Cbc<Aes256, Pkcs7>;

/// This function takes in a password and data, encrypting using AES256. Returns (ciphertext, iv).
pub fn encrypt(password: &String, plaintext: &String) -> (Vec<u8>, Vec<u8>) {
    let password_copy = password.clone();
    let plaintext_copy = plaintext.clone();

    let mut rng = rand::thread_rng();
    let mut hasher = Sha256::new();

    let mut iv = Vec::new();
    // Initialization vector must be 16 bytes long, randomly generated every time. It is not a secret.
    for _i in 0..16 {
        let new_gen: u8 = rng.gen();
        iv.push(new_gen);
    }

    // Put the password into the hasher
    hasher.update(password_copy.as_bytes());
    // Get the hashed password
    let hashed_key = hasher.finalize();
    // Create a new cipher using the hashed password and the initialization vector
    let cipher = Aes256Cbc::new_var(&hashed_key[..], &iv).unwrap();

    // Encrypt
    let ciphertext = cipher.encrypt_vec(plaintext_copy.as_bytes());

    return (ciphertext, iv);

}

/// This function takes in a password, ciphertext, and iv, decrypting using AES256. Returns plaintext.
pub fn decrypt(password: &String, ciphertext: &Vec<u8>, iv: &Vec<u8>) -> Result<String, BlockModeError> {
    let password_copy = password.clone();
    let ciphertext_copy = ciphertext.clone();
    let iv_copy = iv.clone();

    let mut hasher = Sha256::new();


    hasher.update(password_copy.as_bytes());
    // Get the hashed password
    let hashed_key = hasher.finalize();

    // Create a new cipher using the hashed password and the initialization vector
    let cipher = Aes256Cbc::new_var(&hashed_key[..], &iv_copy[..]).unwrap();
    // Decrypt
    let plaintext = cipher.decrypt_vec(&ciphertext_copy[..])?;

    return Ok(String::from_utf8(plaintext).unwrap());
}

#[cfg(test)]
mod tests{
    use super::{encrypt, decrypt};

    /// Checks if the same message encrypted twice with the same password does not give the same outcome/
    #[test]
    fn test_encrypt_twice() {
        let password = "abcd".to_string();
        let plaintext = "this is a plaintext!".to_string();
    
        let tuple1 = encrypt(&password, &plaintext);
        let tuple2 = encrypt(&password, &plaintext);

        assert_ne!(tuple1.0, tuple2.0);
    }

    /// Checks if encrypting and decrypting returns the same plaintext.
    #[test]
    fn test_encrypt_decrypt() {
        let password = "abcd".to_string();
        let plaintext = "this is a plaintext!".to_string();

        let tuple = encrypt(&password, &plaintext);
        let decrypted = decrypt(&password, &tuple.0, &tuple.1).unwrap();

        assert_eq!(plaintext, decrypted);
    }

    /// Checks if a corrupted ciphertext throws an error.
    #[test]
    fn test_corrupted_decrypt() {
        let password = "abcd".to_string();
        let plaintext = "this is a plaintext!".to_string();
        let mut tuple = encrypt(&password, &plaintext);

        let vec_len = tuple.0.len()-1;

        tuple.0[vec_len] = b'1';

        let decrypted = decrypt(&password, &tuple.0, &tuple.1);

        // Error should be thrown, since there is an invalid byte.
        match decrypted {
            Ok(_) => assert!(false),
            Err(_) => assert!(true),
        }
    }

}