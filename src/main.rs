mod api;
mod models;
mod repositories;

use actix_web::{App, web::Data, HttpServer};
use api::question_api::{create_question, get_question, update_question, delete_question, get_questions};
use repositories::question_repository::QuestionRepository;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = QuestionRepository::init().await; // variable db pour établir la connexion avec MongoDB
    let db_data = Data::new(db); // état de la base de donnée disponible dans le code de l'appication
    HttpServer::new(move || {
        App::new()
        .app_data(db_data.clone())
        .service(create_question) // service api question
        .service(get_question)
        .service(update_question)
        .service(delete_question)
        .service(get_questions)
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}