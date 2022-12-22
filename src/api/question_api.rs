use crate::{models::question_model::Question, repositories::question_repository::QuestionRepository};
use actix_web::{
    post, 
    get,
    put,
    web::{Data, Json, Path},
    HttpResponse, delete,
};
use mongodb::bson::oid::ObjectId;

#[post("/question")] // macro routing pour spécifier la méthode HTTP et la route correspondant.
pub async fn create_question(db: Data<QuestionRepository>, new_question: Json<Question>) -> HttpResponse {
    let data = Question {
        id: None,
        category: new_question.category.to_owned(),
        text: new_question.text.to_owned(),
        response: new_question.response.to_owned(),
        suggestions: new_question.suggestions.to_owned(),
    };
    let question_detail = db.create_question(data).await; // insère la question dans la base de données à partie de db.create_question
    match question_detail {
        Ok(question) => HttpResponse::Ok().json(question),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    } // retourne la réponse obtenue suite à la requête
}

#[get("/question/{id}")]
pub async fn get_question(db: Data<QuestionRepository>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    }
    let question_detail = db.get_question(&id).await;
    match question_detail {
        Ok(question) => HttpResponse::Ok().json(question),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

/**
 * * met à jour les données d'une question à partir de son identifiant.
 */
#[put("/question/{id}")]
pub async fn update_question(db: Data<QuestionRepository>, path: Path<String>, new_question: Json<Question>) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    }
    let data = Question {
        id: Some(ObjectId::parse_str(&id).unwrap()),
        category: new_question.category.to_owned(),
        text: new_question.text.to_owned(),
        response: new_question.response.to_owned(),
        suggestions: new_question.suggestions.to_owned(),
    };
    let update_result = db.update_question(&id, data).await;
    match update_result {
        Ok(update) => {
            if update.matched_count == 1 {
                let updated_question_info = db.get_question(&id).await;
                return match updated_question_info {
                    Ok(question) => HttpResponse::Ok().json(question),
                    Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
                };
            } else {
                return HttpResponse::NotFound().body("No question found with specified ID");
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[delete("/question/{id}")]
pub async fn delete_question(db: Data<QuestionRepository>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    }
    let delete_detail = db.delete_question(&id).await;
    match delete_detail {
        Ok(res) => {
            if res.deleted_count == 1 {
                return HttpResponse::Ok().json("Question successfully deleted!");
            }
            else {
                return HttpResponse::NotFound().json("Aucune question ne porte l'ID spécifié");
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string())
    }
}

#[get("/question")]
pub async fn get_questions(db: Data<QuestionRepository>) -> HttpResponse {
    let question_details = db.get_questions().await;
    match question_details {
        Ok(questions) => return HttpResponse::Ok().json(questions),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string())
    }
}