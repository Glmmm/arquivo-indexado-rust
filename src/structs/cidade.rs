use crate::db::file_manager::Entity;
use std::{io, mem::size_of};

#[derive(Debug, Default)]
pub struct Cidade {
    pub codigo_cidade: u32,
    pub descricao: String,
    pub estado: String,
}

impl Entity for Cidade {
    fn get_key(&self) -> u32 {
        self.codigo_cidade
    }

    fn to_bytes(&self) -> Result<Vec<u8>, io::Error> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.codigo_cidade.to_le_bytes());
        
        bytes.extend_from_slice(&(self.descricao.len() as u32).to_le_bytes());
        bytes.extend_from_slice(self.descricao.as_bytes());

        bytes.extend_from_slice(&(self.estado.len() as u32).to_le_bytes());
        bytes.extend_from_slice(self.estado.as_bytes());

        Ok(bytes)
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, io::Error> {
        let mut cursor = 0;
        let  read_u32 = |c: &mut usize| -> u32 {
            let val = u32::from_le_bytes([bytes[*c], bytes[*c + 1], bytes[*c + 2], bytes[*c + 3]]);
            *c += size_of::<u32>();
            val
        };

        let  read_string = |c: &mut usize| -> Result<String, io::Error> {
            let len = read_u32(c) as usize;
            let start = *c;
            let end = start + len;
            let s = String::from_utf8(bytes[start..end].to_vec())
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
            *c = end;
            Ok(s)
        };

        let codigo_cidade = read_u32(&mut cursor);
        let descricao = read_string(&mut cursor)?;
        let estado = read_string(&mut cursor)?;
        
        Ok(Cidade { codigo_cidade, descricao, estado })
    }
}