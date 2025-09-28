use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Exame {
    pub codigo_exame: u32,
    pub descricao: String,
    pub codigo_especialidade: u32,
    pub valor_exame: f32,
}