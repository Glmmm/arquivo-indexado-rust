use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Consulta {
    pub codigo_consulta: u32,
    pub codigo_paciente: u32,
    pub codigo_medico: u32,
    pub codigo_exame: u32,
    pub data: String, 
    pub hora: String,
}