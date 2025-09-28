use crate::db::file_manager::Entity;
use std::io::{self};
use std::mem::size_of;

#[derive(Debug, Default)]
pub struct Medico {
    pub codigo_medico: u32,
    pub nome: String,
    pub endereco: String,
    pub telefone: String,
    pub codigo_cidade: u32,
    pub codigo_especialidade: u32,
}

impl Entity for Medico {
    fn get_key(&self) -> u32 {
        self.codigo_medico
    }

    fn to_bytes(&self) -> Result<Vec<u8>, io::Error> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.codigo_medico.to_le_bytes());
        
        bytes.extend_from_slice(&(self.nome.len() as u32).to_le_bytes());
        bytes.extend_from_slice(self.nome.as_bytes());

        bytes.extend_from_slice(&(self.endereco.len() as u32).to_le_bytes());
        bytes.extend_from_slice(self.endereco.as_bytes());

        bytes.extend_from_slice(&(self.telefone.len() as u32).to_le_bytes());
        bytes.extend_from_slice(self.telefone.as_bytes());

        bytes.extend_from_slice(&self.codigo_cidade.to_le_bytes());
        bytes.extend_from_slice(&self.codigo_especialidade.to_le_bytes());

        Ok(bytes)
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, io::Error> {
        let mut cursor = 0;
        let read_u32 = |c: &mut usize| -> u32 {
            let val = u32::from_le_bytes([bytes[*c], bytes[*c + 1], bytes[*c + 2], bytes[*c + 3]]);
            *c += size_of::<u32>();
            val
        };

        let read_string = |c: &mut usize| -> Result<String, io::Error> {
            let len = read_u32(c) as usize;
            let start = *c;
            let end = start + len;
            let s = String::from_utf8(bytes[start..end].to_vec())
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
            *c = end;
            Ok(s)
        };

        let codigo_medico = read_u32(&mut cursor);
        let nome = read_string(&mut cursor)?;
        let endereco = read_string(&mut cursor)?;
        let telefone = read_string(&mut cursor)?;
        let codigo_cidade = read_u32(&mut cursor);
        let codigo_especialidade = read_u32(&mut cursor);
        
        Ok(Medico { codigo_medico, nome, endereco, telefone, codigo_cidade, codigo_especialidade })
    }
}