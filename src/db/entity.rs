use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, Read, Seek, SeekFrom, Write};

use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::structs::tree::BinaryTree;


pub struct FileManager<T: Serialize + DeserializeOwned + std::fmt::Debug> {
    data_file_path: String,
    index_tree: BinaryTree,
    _phantom: std::marker::PhantomData<T>, 
}

impl<T: Serialize + DeserializeOwned + std::fmt::Debug> FileManager<T> {
    pub fn new(data_file_path: &str) -> io::Result<Self> {
        OpenOptions::new()
            .create(true)
            .write(true)
            .read(true)
            .open(data_file_path)?;

        let mut file_manager = FileManager {
            data_file_path: data_file_path.to_string(),
            index_tree: BinaryTree::new(),
            _phantom: std::marker::PhantomData,
        };

        file_manager.load_index()?;

        Ok(file_manager)
    }

    fn load_index(&mut self) -> io::Result<()> {
        let mut file = File::open(&self.data_file_path)?;
        let mut offset = 0;
        let mut buffer = Vec::new();

        file.read_to_end(&mut buffer)?;

        let content = String::from_utf8(buffer).unwrap_or_default();
        let lines = content.split('\n');

        for line in lines {
            if line.is_empty() {
                continue;
            }
            let record: serde_json::Value = serde_json::from_str(line)?;
            if let Some(key) = record["codigo_paciente"].as_u64() {
                self.index_tree.insert(key as u32, offset);
            }
            offset += (line.len() + 1) as u64; 
        }
        Ok(())
    }

    pub fn create_record(&mut self, record: &T, key: u32) -> io::Result<()> {
        let mut file = OpenOptions::new()
            .append(true)
            .open(&self.data_file_path)?;

        let serialized = serde_json::to_string(record)?;
        let offset = file.seek(SeekFrom::End(0))?;
        
        file.write_all(serialized.as_bytes())?;
        file.write_all(b"\n")?;

        self.index_tree.insert(key, offset);
        Ok(())
    }

    pub fn read_record(&self, key: u32) -> io::Result<Option<T>> {
        if let Some(offset) = self.index_tree.search(key) {
            let mut file = File::open(&self.data_file_path)?;
            file.seek(SeekFrom::Start(offset))?;

            let mut reader = io::BufReader::new(file);
            let mut line = String::new();
            reader.read_line(&mut line)?;
            
            let record: T = serde_json::from_str(&line)?;
            Ok(Some(record))
        } else {
            Ok(None) 
        }
    }
    pub fn delete_record(&mut self, key: u32) -> io::Result<bool> {
        let was_deleted = self.index_tree.delete(key);
        Ok(was_deleted)
    }

    pub fn read_all_records(&self) -> io::Result<Vec<T>> {
        let mut file = File::open(&self.data_file_path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        let mut records = Vec::new();
        for line in content.lines() {
            if !line.trim().is_empty() {
                if let Ok(record) = serde_json::from_str::<T>(line) {
                    records.push(record);
                }
            }
        }
        Ok(records)
    }
}