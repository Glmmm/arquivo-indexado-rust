use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Especialidade {
    pub codigo_especialidade: u32,
    pub descricao: String,
    pub valor_consulta: f32,
    pub limite_diario: u32,
}