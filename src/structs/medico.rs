use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Medico {
    pub codigo_medico: u32,
    pub nome: String,
    pub endereco: String,
    pub telefone: String,
    pub codigo_cidade: u32,
    pub codigo_especialidade: u32,
}