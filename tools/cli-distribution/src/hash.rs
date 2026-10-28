use std::{fs::File, io::Read, path::Path};

use sha2::{Digest, Sha256};

use crate::error::{Error, Result};

pub(crate) fn sha256(path: &Path) -> Result<(u64, String)> {
    let mut file = File::open(path).map_err(|source| Error::Read {
        path: path.to_path_buf(),
        source,
    })?;
    let mut digest = Sha256::new();
    let mut size = 0_u64;
    let mut buffer = [0_u8; 64 * 1024];
    loop {
        let count = file.read(&mut buffer).map_err(|source| Error::Read {
            path: path.to_path_buf(),
            source,
        })?;
        if count == 0 {
            break;
        }
        digest.update(&buffer[..count]);
        size += count as u64;
    }
    Ok((size, format!("{:x}", digest.finalize())))
}

pub(crate) fn is_sha256(value: &str) -> bool {
    value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}
