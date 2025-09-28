use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Cidade {
    pub codigo_cidade: u32,
    pub descricao: String,
    pub estado: String,
}