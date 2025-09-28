use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Diaria {
    pub codigo_dia: u32,
    pub codigo_especialidade: u32,
    pub quantidade_consultas: u32,
}