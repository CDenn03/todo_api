use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use crate::controllers::ApiDoc;

mod controllers;
mod models;
mod services;
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let bind = "127.0.0.1:8080";
    let pool = web::Data::new(services::todo::establish_connection_pool());

    println!("Server running at http://{}", bind);
    HttpServer::new(move || {
        App::new()
            .app_data(pool.clone())
            .route("/todos", web::post().to(controllers::todos::create_todo))
            .route("/todos", web::get().to(controllers::todos::get_todos))
            .route("/todos/{id}", web::get().to(controllers::todos::get_todo))
            .route("/todos/{id}", web::put().to(controllers::todos::update_todo))
            .route("/todos/{id}", web::delete().to(controllers::todos::delete_todo))
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
    })
    .bind(bind)?
    .run()
    .await
}