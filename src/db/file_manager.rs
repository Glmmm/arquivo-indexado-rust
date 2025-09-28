use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write, Seek, SeekFrom};
use std::marker::PhantomData;

use crate::db::tree::BinaryTree;

pub trait Entity {
    fn get_key(&self) -> u32;
    fn to_bytes(&self) -> Result<Vec<u8>, io::Error>;
    fn from_bytes(bytes: &[u8]) -> Result<Self, io::Error> where Self: Sized;
}

pub struct FileManager<T: Entity> {
    file: File,
    index: BinaryTree,
    _phantom: PhantomData<T>,
}

impl<T: Entity> FileManager<T> {
    pub fn new(file_path: &str) -> Result<FileManager<T>, io::Error> {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(file_path)?;

        let mut index = BinaryTree::new();
        let mut offset = 0;
        let mut buffer = Vec::new();

        loop {
            let mut header_buf = [0u8; 5];
            match file.read_exact(&mut header_buf) {
                Ok(_) => {
                    let is_active = header_buf[0];
                    let size = u32::from_le_bytes([header_buf[1], header_buf[2], header_buf[3], header_buf[4]]);
                    
                    buffer.resize(size as usize, 0);
                    file.read_exact(&mut buffer)?;

                    if is_active == 1 {
                        let record = T::from_bytes(&buffer)?;
                        index.insert(record.get_key(), offset);
                    }
                    offset = file.seek(SeekFrom::Current(0))?;
                },
                Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => break, 
                Err(e) => return Err(e),
            }
        }
        
        Ok(FileManager {
            file,
            index,
            _phantom: PhantomData,
        })
    }
    
    pub fn create_record(&mut self, record: &T, key: u32) -> Result<(), io::Error> {
        if self.index.search(key).is_some() {
            return Err(io::Error::new(io::ErrorKind::AlreadyExists, "Record with this key already exists."));
        }

        let serialized_data = record.to_bytes()?;
        let size = serialized_data.len() as u32;

        let offset = self.file.seek(SeekFrom::End(0))?;
        
        self.file.write_all(&[1])?;
        self.file.write_all(&size.to_le_bytes())?;
        
        self.file.write_all(&serialized_data)?;

        self.index.insert(key, offset);
        Ok(())
    }

    pub fn read_record(&self, key: u32) -> Result<Option<T>, io::Error> {
        if let Some(offset) = self.index.search(key) {
            let mut file = self.file.try_clone()?;
            file.seek(SeekFrom::Start(offset))?;
            
            let mut header_buf = [0u8; 5];
            file.read_exact(&mut header_buf)?;

            let is_active = header_buf[0];
            let size = u32::from_le_bytes([header_buf[1], header_buf[2], header_buf[3], header_buf[4]]);

            if is_active != 1 {
                return Ok(None);
            }

            let mut buffer = Vec::new();
            buffer.resize(size as usize, 0);
            file.read_exact(&mut buffer)?;

            Ok(Some(T::from_bytes(&buffer)?))
        } else {
            Ok(None)
        }
    }
    
    pub fn delete_record(&mut self, key: u32) -> Result<bool, io::Error> {
        if let Some(offset) = self.index.search(key) {
            self.file.seek(SeekFrom::Start(offset))?;
            self.file.write_all(&[0])?; 
            self.index.delete(key);
            Ok(true)
        } else {
            Ok(false)
        }
    }
    
    pub fn read_all_records(&self) -> Result<Vec<T>, io::Error> {
        let mut records = Vec::new();
        let mut file = self.file.try_clone()?;
        file.seek(SeekFrom::Start(0))?;
        
        let mut buffer = Vec::new();
        
        loop {
            let mut header_buf = [0u8; 5];
            match file.read_exact(&mut header_buf) {
                Ok(_) => {
                    let is_active = header_buf[0];
                    let size = u32::from_le_bytes([header_buf[1], header_buf[2], header_buf[3], header_buf[4]]);
                    
                    buffer.resize(size as usize, 0);
                    file.read_exact(&mut buffer)?;

                    if is_active == 1 {
                        records.push(T::from_bytes(&buffer)?);
                    }
                },
                Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => break,
                Err(e) => return Err(e),
            }
        }
        
        Ok(records)
    }
}