use std::{collections::HashMap, path::{Path, PathBuf}};

use sha2::{Sha256, Digest};
use serde::{Deserialize, Serialize};
use std::io::{self, Read, Write};

use tokio::fs as async_fs;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ContentHash(String);

impl ContentHash {
    pub fn from_bytes(data: &[u8]) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(data);
        let result = hasher.finalize();
        ContentHash(hex::encode(result))
    }

    pub fn from_str(data: &str) -> Self {
        let bytes = data.as_bytes();
        Self::from_bytes(bytes)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}


pub struct SimpleStore {
    objects: HashMap<ContentHash, Vec<u8>>,
}

impl SimpleStore {
    pub fn new() -> Self {
        Self {
            objects: HashMap::new()
        }
    }

    pub fn put(&mut self, data: Vec<u8>) -> ContentHash {
        let hash = ContentHash::from_bytes(&data);
        self.objects.insert(hash.clone(), data);
        hash
    }


    pub fn get(&self, hash: &ContentHash) -> Option<&Vec<u8>> {
        self.objects.get(hash)
    }

    pub fn contains(&self, hash: &ContentHash) -> bool {
        self.objects.contains_key(hash)
    }
}



pub struct PersistentStore {
    root: PathBuf,
}

impl PersistentStore {
    pub fn new<P: AsRef<Path>>(root: P) -> io::Result<Self> {
        let root = root.as_ref().to_path_buf();
        std::fs::create_dir_all(&root)?;
        Ok(Self { root })
    } 

    fn hash_to_path(&self, hash: &ContentHash) -> PathBuf {
        let hash_str = hash.as_str();
        let mut path = self.root.clone();
        path.push("objects");

        // Use first 4 characters for nested directory structure
        // This gives us 16^4 = 65,536 possible directories so that the files can be spread out evenly 

        if hash_str.len() >= 4 {
            path.push(&hash_str[0..2]);
            path.push(&hash_str[2..4]);
            path.push(&hash_str[4..]);
        } else {
            path.push(hash_str);
        }
        path
    }

    pub async fn put(&self, data: Vec<u8>) -> io::Result<ContentHash> {
        let hash = ContentHash::from_bytes(&data);
        let file_path = self.hash_to_path(&hash);

        // check if file already exists (deduplication)
        if file_path.exists() {
            return Ok(hash);
        }

        // create parent directories
        if let Some(parent) = file_path.parent() {
            async_fs::create_dir_all(parent).await?;
        }


        // write data atomically using temporary file 
        let temp_path = file_path.with_extension("tmp");
        async_fs::write(&temp_path, &data).await?;
        async_fs::rename(&temp_path, &file_path).await?;

        Ok(hash)
    }


    pub async fn get(&self, hash: &ContentHash) -> io::Result<Option<Vec<u8>>> {
        let file_path = self.hash_to_path(hash);

        match async_fs::read(file_path).await {
            Ok(data) => {
                // verify content integrity 
                let computed_hash = ContentHash::from_bytes(&data);

                if computed_hash == *hash {
                    Ok(Some(data))
                } else {
                    // corruption detected
                    Err(io::Error::new(io::ErrorKind::InvalidData, "corrupted data"))
                }
            }
            Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(e),
        }
    }


    pub async fn contains(&self, hash: &ContentHash) -> bool {
        self.hash_to_path(hash).exists()
    }


}



#[derive(Debug, Serialize, Deserialize, Clone)]
struct ChunkedFile {
    total_size: u64,
    chunks: Vec<ContentHash>,
}

pub struct ChunkedStore {
    root: PathBuf,
}

const CHUNK_SIZE: usize = 1024 * 64; // 64KB

impl ChunkedStore {
    pub fn new<P: AsRef<Path>>(root: P) -> io::Result<Self> {
        let root = root.as_ref().to_path_buf();
        std::fs::create_dir_all(&root)?;
        Ok(Self { root })
    }


    pub async fn store_file(&self, data: Vec<u8>) -> io::Result<ContentHash> {
        if data.len() <= CHUNK_SIZE {
            // small file: store directly
            return self.store_chunk(data).await;
        };

        // large file: split into chunks
        let mut chunk_hashes = Vec::new();

        for chunk_data in data.chunks(CHUNK_SIZE) {
            let chunk_hash = self.store_chunk(chunk_data.to_vec()).await?;
            chunk_hashes.push(chunk_hash);
        };

        // create metadata pointing to all chunks
        let chunked_file = ChunkedFile {
            chunks: chunk_hashes,
            total_size: data.len() as u64
        };

        // store metadata
        let metadata = bincode::serialize(&chunked_file).unwrap();
        self.store_chunk(metadata).await
    }

    async fn store_chunk(&self, data: Vec<u8>) -> io::Result<ContentHash> {
        let hash = ContentHash::from_bytes(&data);
        let file_path = self.chunk_path(&hash);

        if file_path.exists() {
            return Ok(hash);
        }

        if let Some(parent) = file_path.parent() {
            async_fs::create_dir_all(parent).await?;
        }

        async_fs::write(file_path, data).await?;
        Ok(hash)
    }

    pub async fn get_file(&self, hash: &ContentHash) -> io::Result<Option<Vec<u8>>> {
        let chunk_data = match self.get_chunk(hash).await? {
            Some(data) => data,
            None => return Ok(None),
        };

        // try to parse as chunked file metadata
        if let Ok(chunked_file) = bincode::deserialize::<ChunkedFile>(&chunk_data) {
            let mut result = Vec::with_capacity(chunked_file.total_size as usize);

            for chunk_hash in &chunked_file.chunks {
                if let Some(chunk) = self.get_chunk(chunk_hash).await? {
                    result.extend_from_slice(&chunk);
                } else {
                    return Err(io::Error::new(io::ErrorKind::NotFound, "missing chunk"));
                }
            }
            Ok(Some(result))
        } else {
            Ok(Some(chunk_data))
        }
    }

    async fn get_chunk(&self, hash: &ContentHash) -> io::Result<Option<Vec<u8>>> {
        let file_path = self.chunk_path(hash);
        match async_fs::read(file_path).await {
            Ok(data) => Ok(Some(data)),
            Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(e)
        }
    }


    fn chunk_path(&self, hash: &ContentHash) -> PathBuf {
        let hash_str = hash.as_str();
        let mut path = self.root.clone();
        path.push("chunks");
        path.push(&hash_str[0..2]);
        path.push(&hash_str[2..]);
        path
    }
}




#[cfg(test)]

mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_chunked_storage() {
        let temp_dir = TempDir::new().unwrap();
        let store = ChunkedStore::new(temp_dir.path()).unwrap();

        let large_file = vec![42u8; 200 * 1024];
        let hash = store.store_file(large_file.clone()).await.unwrap();

        let retrieved = store.get_file(&hash).await.unwrap().unwrap();
        assert_eq!(retrieved, large_file);

        let hash2 = store.store_file(large_file.clone()).await.unwrap();
        assert_eq!(hash, hash2);

        let mut modified_file = large_file.clone();
        modified_file[1000] = 99;

        let hash3 = store.store_file(modified_file).await.unwrap();
        assert_ne!(hash, hash3);

    }

}