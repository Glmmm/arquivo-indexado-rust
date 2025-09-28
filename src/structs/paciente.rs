use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Paciente {
    pub codigo_paciente: u32,
    pub nome: String,
    pub data_nascimento: String,
    pub endereco: String,
    pub telefone: String,
    pub codigo_cidade: u32,
    pub peso: f32,
    pub altura: f32,
}