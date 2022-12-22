use std::env;
extern crate dotenv;
use dotenv::dotenv;

use futures::TryStreamExt;
use mongodb::{
    bson::{extjson::de::Error, oid::ObjectId, doc},
    results::{InsertOneResult, UpdateResult, DeleteResult},
    Client, Collection,
};

use crate::models::question_model::Question;

pub struct QuestionRepository {
    question_collection: Collection<Question>,
}

impl QuestionRepository {
    pub async fn init() -> Self {
        dotenv().ok(); // charger les variables environnement
        let uri = match env::var("MONGOURI") { 
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        }; // récupération de la valeur liée à la clé MONGOURI dans le fichier .env
        let client = Client::with_uri_str(uri).await.unwrap(); // un client connecté à MongoDB
        let db = client.database("quizDB"); // sélection d'une base de données
        let question_collection: Collection<Question> = db.collection("Question"); // sélection d'une collection en fonction de la base de données
        QuestionRepository {question_collection}
    }

    /**
     * enregistre une nouvelle question dans la base de données.
     * retourne la question créée, sinon une erreur.
     */
    pub async fn create_question(&self, new_question: Question) -> Result<InsertOneResult, Error> {
        let new_doc = Question {
            id: None, // None permet de dire à MongoDB de générer automatiquement l'ID de la question.
            category: new_question.category,
            text: new_question.text,
            response: new_question.response,
            suggestions: new_question.suggestions,
        };
        let question = self
            .question_collection
            .insert_one(new_doc, None)
            .await
            .ok()
            .expect("Error creating question");
        Ok(question)
    }

    /**
     * get_question
     * * retourne une question par son identifiant en interrogeant la base de données MongoDB.
     * @param self reférence la structure QuestionRepository
     * @param id correspond à l'identifiant de la question recherchée
     */
    pub async fn get_question(&self, id: &String) -> Result<Question, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap(); // conversion de l'identifiant en ObjectID
        let filter = doc! {"_id": obj_id};
        let question_detail = self
            .question_collection
            .find_one(filter, None)
            .await
            .ok()
            .expect("Error getting question's detail");
        Ok(question_detail.unwrap()) // retourne le détail de la question, sinon une erreur.
    }

    /**
     * * met à jour une question par rapport à son identifiant.
     */
    pub async fn update_question(&self, id: &String, new_question: Question) -> Result<UpdateResult, Error>{
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let new_doc = doc! {
            "$set": {
                "id": new_question.id,
                "category": new_question.category,
                "text": new_question.text,
                "response": new_question.response,
                "suggestions": new_question.suggestions
            },
        };
        let updated_doc = self
            .question_collection
            .update_one(filter, new_doc, None)
            .await
            .ok()
            .expect("Error updating question");
        Ok(updated_doc)
    }

    pub async fn delete_question(&self, id: &String) -> Result<DeleteResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let user_detail = self
            .question_collection
            .delete_one(filter, None)
            .await
            .ok()
            .expect("Error deleting question");
        Ok(user_detail)
    }

    pub async fn get_questions(&self) -> Result<Vec<Question>, Error> {
        let mut cursors = self
            .question_collection
            .find(None, None)
            .await
            .ok()
            .expect("Error getting questions");
        let mut questions: Vec<Question> = Vec::new();
        while let Some(question) = cursors
            .try_next()
            .await
            .ok()
            .expect("Error mapping through cursor") 
        {
            questions.push(question)
        }
        Ok(questions)
    }
}