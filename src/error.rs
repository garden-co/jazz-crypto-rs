use std::fmt;

#[derive(Debug)]
pub enum CryptoError {
    InvalidKeyLength,
    InvalidNonceLength,
    InvalidSealerSecretFormat,
    InvalidSignatureLength,
    InvalidVerifyingKey(String),
    InvalidPublicKey(String),
    WrongTag,
    CipherError,
    InvalidPrefix(&'static str, &'static str),
    Base58Error(String),
}

impl fmt::Display for CryptoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CryptoError::InvalidKeyLength => write!(f, "Invalid key length"),
            CryptoError::InvalidNonceLength => write!(f, "Invalid nonce length"),
            CryptoError::InvalidSealerSecretFormat => {
                write!(
                    f,
                    "Invalid sealer secret format: must start with 'sealerSecret_z'"
                )
            }
            CryptoError::InvalidSignatureLength => write!(f, "Invalid signature length"),
            CryptoError::InvalidVerifyingKey(e) => write!(f, "Invalid verifying key: {}", e),
            CryptoError::InvalidPublicKey(e) => write!(f, "Invalid public key: {}", e),
            CryptoError::WrongTag => write!(f, "Wrong tag"),
            CryptoError::CipherError => write!(f, "Failed to create cipher"),
            CryptoError::InvalidPrefix(prefix, field) => {
                write!(f, "Invalid {} format: must start with '{}'", field, prefix)
            }
            CryptoError::Base58Error(e) => write!(f, "Invalid base58: {}", e),
        }
    }
}

impl std::error::Error for CryptoError {}
