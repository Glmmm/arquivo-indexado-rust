use crate::db::file_manager::Entity;
use std::io::{self};
use std::mem::size_of;

#[derive(Debug, Default)]
pub struct Consulta {
    pub codigo_consulta: u32,
    pub codigo_paciente: u32,
    pub codigo_medico: u32,
    pub codigo_exame: u32,
    pub data: String, //AAAAMMDD
    pub hora: String, //HH:MM
}

impl Entity for Consulta {
    fn get_key(&self) -> u32 {
        self.codigo_consulta
    }

    fn to_bytes(&self) -> Result<Vec<u8>, io::Error> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.codigo_consulta.to_le_bytes());
        bytes.extend_from_slice(&self.codigo_paciente.to_le_bytes());
        bytes.extend_from_slice(&self.codigo_medico.to_le_bytes());
        bytes.extend_from_slice(&self.codigo_exame.to_le_bytes());

        bytes.extend_from_slice(&(self.data.len() as u32).to_le_bytes());
        bytes.extend_from_slice(self.data.as_bytes());

        bytes.extend_from_slice(&(self.hora.len() as u32).to_le_bytes());
        bytes.extend_from_slice(self.hora.as_bytes());

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

        let codigo_consulta = read_u32(&mut cursor);
        let codigo_paciente = read_u32(&mut cursor);
        let codigo_medico = read_u32(&mut cursor);
        let codigo_exame = read_u32(&mut cursor);
        let data = read_string(&mut cursor)?;
        let hora = read_string(&mut cursor)?;

        Ok(Consulta {
            codigo_consulta,
            codigo_paciente,
            codigo_medico,
            codigo_exame,
            data,
            hora,
        })
    }
}
