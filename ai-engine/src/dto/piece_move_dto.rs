use pyo3::prelude::*;

#[derive(Debug, Clone)]
#[pyclass]
pub struct PieceMoveDTO {
    #[pyo3(get)]
    pub from_position: i8,
    #[pyo3(get)]
    pub is_capture: bool,
    #[pyo3(get)]
    pub is_en_passant: bool,
    #[pyo3(get)]
    pub is_promotion: bool,
    #[pyo3(get)]
    pub piece_value: i8,
    #[pyo3(get)]
    pub promotion_type: char,
    #[pyo3(get)]
    pub to_position: i8,
}

#[pymethods]
impl PieceMoveDTO {
    #[new]
    pub fn new(
        from_position: i8,
        is_capture: bool,
        is_en_passant: bool,
        is_promotion: bool,
        piece_value: i8, 
        promotion_type: char,
        to_position: i8,
    ) -> Self {
        Self {
            from_position,
            is_capture,
            is_en_passant,
            is_promotion,
            piece_value,
            promotion_type,
            to_position,
        }
    }
}
