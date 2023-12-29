use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct FenDTO {
    pub fen: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MovesCountDTO {
    pub depth: usize
}