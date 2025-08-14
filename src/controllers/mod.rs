use actix_web::{web, HttpResponse, Responder};
use crate::models;
use crate::services::todo::DbPool;
use utoipa::OpenApi;

#[utoipa::path(
    post,
    path = "/todos",
    request_body = models::TodoInput,
    responses(
        (status = 200, description = "Created a new TODO", body = models::Todo),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn create_todo(
    todo: web::Json<models::TodoInput>,
    pool: web::Data<DbPool>,
) -> impl Responder {
    match crate::services::todo::create_todo(&pool, todo.into_inner()) {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(_) => HttpResponse::InternalServerError().body("Error saving new todo"),
    }
}

#[utoipa::path(
    get,
    path = "/todos",
    responses(
        (status = 200, description = "Get all TODOs", body = Vec<models::Todo>),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_todos(pool: web::Data<DbPool>) -> impl Responder {
    match crate::services::todo::get_todos(&pool) {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(_) => HttpResponse::InternalServerError().body("Error loading todos"),
    }
}

#[utoipa::path(
    get,
    path = "/todos/{id}",
    params(
        ("id", description = "ID of the TODO to retrieve")
    ),
    responses(
        (status = 200, description = "Get a specific TODO", body = models::Todo),
        (status = 404, description = "Todo not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_todo(
    path: web::Path<i32>,
    pool: web::Data<DbPool>,
) -> impl Responder {
    let todo_id = path.into_inner();
    match crate::services::todo::get_todo(&pool, todo_id) {
        Ok(Some(todo)) => HttpResponse::Ok().json(todo),
        Ok(None) => HttpResponse::NotFound().body("Todo not found"),
        Err(_) => HttpResponse::InternalServerError().body("Error loading todo"),
    }
}

#[utoipa::path(
    put,
    path = "/todos/{id}",
    params(
        ("id", description = "ID of the TODO to update")
    ),
    request_body = models::TodoInput,
    responses(
        (status = 200, description = "Update a TODO", body = models::Todo),
        (status = 404, description = "Todo not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn update_todo(
    path: web::Path<i32>,
    todo: web::Json<models::TodoInput>,
    pool: web::Data<DbPool>,
) -> impl Responder {
    let todo_id = path.into_inner();
    match crate::services::todo::update_todo(&pool, todo_id, todo.title.clone()) {
        Ok(Some(updated_todo)) => HttpResponse::Ok().json(updated_todo),
        Ok(None) => HttpResponse::NotFound().body("Todo not found"),
        Err(_) => HttpResponse::InternalServerError().body("Error updating todo"),
    }
}

#[utoipa::path(
    delete,
    path = "/todos/{id}",
    params(
        ("id", description = "ID of the TODO to delete")
    ),
    responses(
        (status = 200, description = "Todo deleted"),
        (status = 404, description = "Todo not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn delete_todo(
    path: web::Path<i32>,
    pool: web::Data<DbPool>,
) -> impl Responder {
    let todo_id = path.into_inner();
    match crate::services::todo::delete_todo(&pool, todo_id) {
        Ok(count) if count > 0 => HttpResponse::Ok().body("Todo deleted"),
        Ok(_) => HttpResponse::NotFound().body("Todo not found"),
        Err(_) => HttpResponse::InternalServerError().body("Error deleting todo"),
    }
}

#[derive(OpenApi)]
#[openapi(
    paths(
        create_todo,
        get_todos,
        get_todo,
        update_todo,
        delete_todo
    ),
    components(
        schemas(crate::models::Todo, crate::models::TodoInput)
    )
)]
pub struct ApiDoc;

pub mod todos {
    pub use super::{
        create_todo,
        get_todos,
        get_todo,
        update_todo,
        delete_todo,
    };
}