use cuid::CuidError;
use sha3::{Digest, Sha3_512};

pub fn create_token() -> Result<String, CuidError> {
    let cuid_base_hash = cuid::cuid()?;
    let mut hasher = Sha3_512::new();
    hasher.update(cuid_base_hash.as_bytes());
    let token = hasher.finalize();
    let hex = hex::encode(token);
    Ok(hex)
}
