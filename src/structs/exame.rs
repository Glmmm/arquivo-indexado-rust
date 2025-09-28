use crate::db::file_manager::Entity;
use std::io::{self};
use std::mem::size_of;

#[derive(Debug, Default)]
pub struct Exame {
    pub codigo_exame: u32,
    pub descricao: String,
    pub codigo_especialidade: u32,
    pub valor_exame: f32,
}

impl Entity for Exame {
    fn get_key(&self) -> u32 {
        self.codigo_exame
    }

    fn to_bytes(&self) -> Result<Vec<u8>, io::Error> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.codigo_exame.to_le_bytes());
        
        bytes.extend_from_slice(&(self.descricao.len() as u32).to_le_bytes());
        bytes.extend_from_slice(self.descricao.as_bytes());

        bytes.extend_from_slice(&self.codigo_especialidade.to_le_bytes());
        bytes.extend_from_slice(&self.valor_exame.to_le_bytes());
        
        Ok(bytes)
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self, io::Error> {
        let mut cursor = 0;
        let read_u32 = |c: &mut usize| -> u32 {
            let val = u32::from_le_bytes([bytes[*c], bytes[*c + 1], bytes[*c + 2], bytes[*c + 3]]);
            *c += size_of::<u32>();
            val
        };
        let read_f32 = |c: &mut usize| -> f32 {
            let val = f32::from_le_bytes([bytes[*c], bytes[*c + 1], bytes[*c + 2], bytes[*c + 3]]);
            *c += size_of::<f32>();
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

        let codigo_exame = read_u32(&mut cursor);
        let descricao = read_string(&mut cursor)?;
        let codigo_especialidade = read_u32(&mut cursor);
        let valor_exame = read_f32(&mut cursor);
        
        Ok(Exame { codigo_exame, descricao, codigo_especialidade, valor_exame })
    }
}