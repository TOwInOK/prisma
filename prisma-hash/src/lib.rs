use sha1::{Digest, Sha1};
use sha2::{Sha256, Sha512};
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum HashType {
    SHA1(String),
    SHA512(String),
    SHA256(String),
    MD5(String),
    None,
}

impl HashType {
    pub fn compute_md5(data: impl AsRef<[u8]>) -> String {
        let digest = md5::compute(data.as_ref());
        format!("{:x}", digest)
    }

    pub fn compute_sha1(data: impl AsRef<[u8]>) -> String {
        let mut hasher = Sha1::new();
        hasher.update(data.as_ref());
        format!("{:x}", hasher.finalize())
    }

    pub fn compute_sha512(data: impl AsRef<[u8]>) -> String {
        let mut hasher = Sha512::new();
        hasher.update(data.as_ref());
        format!("{:x}", hasher.finalize())
    }

    pub fn compute_sha256(data: impl AsRef<[u8]>) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data.as_ref());
        format!("{:x}", hasher.finalize())
    }

    // Constructor methods
    pub fn new_md5(hash: String) -> Self {
        HashType::MD5(hash)
    }

    pub fn new_sha1(hash: String) -> Self {
        HashType::SHA1(hash)
    }

    pub fn new_sha512(hash: String) -> Self {
        HashType::SHA512(hash)
    }

    pub fn new_sha256(hash: String) -> Self {
        HashType::SHA256(hash)
    }

    /// Return error if hash is't compare
    pub fn compare(
        &self,
        data: impl AsRef<[u8]>,
    ) -> std::result::Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        match self {
            HashType::SHA1(expected) => {
                let hash = Self::compute_sha1(data);
                if expected == &hash {
                    Ok(())
                } else {
                    Err(format!("Hash mismatch: expected {} but got {}", expected, hash).into())
                }
            }
            HashType::SHA512(expected) => {
                let hash = Self::compute_sha512(data);
                if expected == &hash {
                    Ok(())
                } else {
                    Err(format!("Hash mismatch: expected {} but got {}", expected, hash).into())
                }
            }
            HashType::MD5(expected) => {
                let hash = Self::compute_md5(data);
                if expected == &hash {
                    Ok(())
                } else {
                    Err(format!("Hash mismatch: expected {} but got {}", expected, hash).into())
                }
            }
            HashType::SHA256(expected) => {
                let hash = Self::compute_sha256(data);
                if expected == &hash {
                    Ok(())
                } else {
                    Err(format!("Hash mismatch: expected {} but got {}", expected, hash).into())
                }
            }
            HashType::None => Ok(()),
        }
    }
}

impl std::fmt::Display for HashType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HashType::SHA1(hash) => write!(f, "SHA1: {}", hash),
            HashType::SHA256(hash) => write!(f, "SHA256: {}", hash),
            HashType::SHA512(hash) => write!(f, "SHA512: {}", hash),
            HashType::MD5(hash) => write!(f, "MD5: {}", hash),
            HashType::None => write!(f, "None hash"),
        }
    }
}
