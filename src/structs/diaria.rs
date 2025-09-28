use crate::db::file_manager::Entity;
use std::io::{self};
use std::mem::size_of;

#[derive(Debug, Default)]
pub struct Diaria {
    pub codigo_dia: u32,
    pub codigo_especialidade: u32,
    pub quantidade_consultas: u32,
}

impl Entity for Diaria {
    fn get_key(&self) -> u32 {
        self.codigo_dia
    }

    fn to_bytes(&self) -> Result<Vec<u8>, io::Error> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.codigo_dia.to_le_bytes());
        bytes.extend_from_slice(&self.codigo_especialidade.to_le_bytes());
        bytes.extend_from_slice(&self.quantidade_consultas.to_le_bytes());
        Ok(bytes)
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, io::Error> {
        let mut cursor = 0;
        let read_u32 = |c: &mut usize| -> u32 {
            let val = u32::from_le_bytes([bytes[*c], bytes[*c + 1], bytes[*c + 2], bytes[*c + 3]]);
            *c += size_of::<u32>();
            val
        };

        let codigo_dia = read_u32(&mut cursor);
        let codigo_especialidade = read_u32(&mut cursor);
        let quantidade_consultas = read_u32(&mut cursor);
        
        Ok(Diaria { codigo_dia, codigo_especialidade, quantidade_consultas })
    }
}