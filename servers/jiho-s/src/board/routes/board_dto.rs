use crate::entities::board;
use crate::entities::board::Model;

#[derive(serde::Deserialize)]
pub struct CreateBoardRequestDto {
    pub name: String,
}

#[derive(serde::Serialize)]
pub struct BoardResponseDto {
    pub id: i32,
    pub name: String,
}

impl From<board::Model> for BoardResponseDto {
    fn from(value: board::Model) -> Self {
        BoardResponseDto {
            id: value.id,
            name: value.name,
        }
    }
}

impl From<&board::Model> for BoardResponseDto {
    fn from(value: &Model) -> Self {
        BoardResponseDto {
            id: value.id,
            name: value.name.to_owned(),
        }
    }
}