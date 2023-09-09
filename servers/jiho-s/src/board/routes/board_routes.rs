use actix_web::{delete, get, HttpResponse, post, Scope, web};
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait, ModelTrait, NotSet};
use sea_orm::ActiveValue::Set;

use crate::board::routes::board_dto::{BoardResponseDto, CreateBoardRequestDto};
use crate::entities::board;
use crate::entities::prelude::Board;

#[post("")]
pub async fn add_board(
    json: web::Json<CreateBoardRequestDto>,
    connection: web::Data<DatabaseConnection>,
) -> HttpResponse {
    let new_board_name = json.into_inner().name;
    match insert_board(connection.get_ref(), new_board_name).await {
        Ok(board) => HttpResponse::Ok().json(BoardResponseDto::from(board)),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn insert_board(
    connection: &DatabaseConnection,
    name: String,
) -> Result<board::Model, DbErr> {
    let board = board::ActiveModel {
        id: NotSet,
        name: Set(name),
    };

    board.save(connection).await?.try_into()
}

#[delete("/{board_id}")]
pub async fn delete_board(
    path: web::Path<i32>,
    connection: web::Data<DatabaseConnection>,
) -> HttpResponse {
    let board_id = path.into_inner();
    if remove_board(connection.get_ref(), board_id).await.is_err() {
        return HttpResponse::InternalServerError().finish();
    };
    HttpResponse::Ok().finish()
}

async fn remove_board(
    connection: &DatabaseConnection,
    id: i32,
) -> Result<(), anyhow::Error> {
    let board = Board::find_by_id(id).one(connection).await?
        .expect("Board NotFound");

    board.delete(connection).await?;
    Ok(())
}

#[get("/{board_id}")]
pub async fn get_board(
    path: web::Path<i32>,
    connection: web::Data<DatabaseConnection>,
) -> HttpResponse {
    let board_id = path.into_inner();
    match get_board_by_id(connection.get_ref(), board_id).await {
        Ok(board) => HttpResponse::Ok().json(BoardResponseDto::from(board)),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn get_board_by_id(
    connection: &DatabaseConnection,
    id: i32,
) -> Result<board::Model, DbErr> {
    let option = Board::find_by_id(id).one(connection).await?;
    Ok(option.expect(""))
}

#[get("")]
pub async fn get_boards(
    connection: web::Data<DatabaseConnection>,
) -> HttpResponse {
    match get_all_boards(connection.get_ref()).await {
        Ok(boards) => HttpResponse::Ok().json(boards.iter().map(BoardResponseDto::from).collect::<Vec<BoardResponseDto>>()),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn get_all_boards(
    connection: &DatabaseConnection,
) -> Result<Vec<board::Model>, DbErr> {
    Board::find().all(connection).await
}

pub fn board_scope() -> Scope {
    web::scope("/api/boards")
        .service(add_board)
        .service(delete_board)
        .service(get_board)
        .service(get_boards)
}