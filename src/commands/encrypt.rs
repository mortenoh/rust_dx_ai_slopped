//! Encrypt command - encryption utilities.

use crate::cli::commands::encrypt::{EncryptAlgorithm, EncryptArgs, EncryptCommand};
use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce as AesNonce,
};
use anyhow::{Context, Result};
use base64::Engine;
use chacha20poly1305::{ChaCha20Poly1305, Nonce as ChachaNonce};
use rand::Rng;
use sha2::{Digest, Sha256};
use std::fs;
use std::io::{self, Read, Write};
use std::path::PathBuf;

/// Run the encrypt command
pub fn run(args: EncryptArgs) -> Result<()> {
    match args.command {
        EncryptCommand::Encrypt {
            input,
            string,
            out_file,
            password,
            algorithm,
        } => cmd_encrypt(input, string, out_file, &password, algorithm),
        EncryptCommand::Decrypt {
            input,
            string,
            out_file,
            password,
            algorithm,
        } => cmd_decrypt(input, string, out_file, &password, algorithm),
    }
}

fn read_input(input: Option<PathBuf>, string: Option<String>) -> Result<Vec<u8>> {
    if let Some(s) = string {
        return Ok(s.into_bytes());
    }

    match input {
        Some(path) if path.to_string_lossy() == "-" => {
            let mut buffer = Vec::new();
            io::stdin()
                .read_to_end(&mut buffer)
                .context("Failed to read from stdin")?;
            Ok(buffer)
        }
        Some(path) => {
            fs::read(&path).with_context(|| format!("Failed to read file: {}", path.display()))
        }
        None => {
            let mut buffer = Vec::new();
            io::stdin()
                .read_to_end(&mut buffer)
                .context("Failed to read from stdin")?;
            Ok(buffer)
        }
    }
}

fn write_output(output: Option<PathBuf>, data: &[u8]) -> Result<()> {
    match output {
        Some(path) => {
            fs::write(&path, data)
                .with_context(|| format!("Failed to write file: {}", path.display()))?;
        }
        None => {
            io::stdout()
                .write_all(data)
                .context("Failed to write to stdout")?;
            println!(); // Add newline for terminal output
        }
    }
    Ok(())
}

fn derive_key(password: &str) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    hasher.finalize().into()
}

fn cmd_encrypt(
    input: Option<PathBuf>,
    string: Option<String>,
    output: Option<PathBuf>,
    password: &str,
    algorithm: EncryptAlgorithm,
) -> Result<()> {
    let plaintext = read_input(input, string)?;
    let key = derive_key(password);

    let ciphertext = match algorithm {
        EncryptAlgorithm::Chacha20 => {
            let cipher = ChaCha20Poly1305::new_from_slice(&key)
                .map_err(|e| anyhow::anyhow!("Failed to create cipher: {}", e))?;

            // Generate random nonce (12 bytes for ChaCha20-Poly1305)
            let mut nonce_bytes = [0u8; 12];
            rand::rng().fill(&mut nonce_bytes);
            let nonce = ChachaNonce::from_slice(&nonce_bytes);

            let encrypted = cipher
                .encrypt(nonce, plaintext.as_ref())
                .map_err(|e| anyhow::anyhow!("Encryption failed: {}", e))?;

            // Prepend nonce to ciphertext
            let mut result = nonce_bytes.to_vec();
            result.extend(encrypted);
            result
        }
        EncryptAlgorithm::AesGcm => {
            let cipher = Aes256Gcm::new_from_slice(&key)
                .map_err(|e| anyhow::anyhow!("Failed to create cipher: {}", e))?;

            // Generate random nonce (12 bytes for AES-GCM)
            let mut nonce_bytes = [0u8; 12];
            rand::rng().fill(&mut nonce_bytes);
            let nonce = AesNonce::from_slice(&nonce_bytes);

            let encrypted = cipher
                .encrypt(nonce, plaintext.as_ref())
                .map_err(|e| anyhow::anyhow!("Encryption failed: {}", e))?;

            // Prepend nonce to ciphertext
            let mut result = nonce_bytes.to_vec();
            result.extend(encrypted);
            result
        }
    };

    // Base64 encode for text output
    let encoded = base64::engine::general_purpose::STANDARD.encode(&ciphertext);
    write_output(output, encoded.as_bytes())
}

fn cmd_decrypt(
    input: Option<PathBuf>,
    string: Option<String>,
    output: Option<PathBuf>,
    password: &str,
    algorithm: EncryptAlgorithm,
) -> Result<()> {
    let encoded = read_input(input, string)?;
    let encoded_str = String::from_utf8(encoded).context("Invalid UTF-8 in input")?;

    // Base64 decode
    let ciphertext = base64::engine::general_purpose::STANDARD
        .decode(encoded_str.trim())
        .context("Failed to decode base64")?;

    if ciphertext.len() < 12 {
        anyhow::bail!("Ciphertext too short");
    }

    let key = derive_key(password);

    let plaintext = match algorithm {
        EncryptAlgorithm::Chacha20 => {
            let cipher = ChaCha20Poly1305::new_from_slice(&key)
                .map_err(|e| anyhow::anyhow!("Failed to create cipher: {}", e))?;

            let nonce = ChachaNonce::from_slice(&ciphertext[..12]);
            let encrypted = &ciphertext[12..];

            cipher.decrypt(nonce, encrypted).map_err(|_| {
                anyhow::anyhow!("Decryption failed: invalid password or corrupted data")
            })?
        }
        EncryptAlgorithm::AesGcm => {
            let cipher = Aes256Gcm::new_from_slice(&key)
                .map_err(|e| anyhow::anyhow!("Failed to create cipher: {}", e))?;

            let nonce = AesNonce::from_slice(&ciphertext[..12]);
            let encrypted = &ciphertext[12..];

            cipher.decrypt(nonce, encrypted).map_err(|_| {
                anyhow::anyhow!("Decryption failed: invalid password or corrupted data")
            })?
        }
    };

    write_output(output, &plaintext)
}
