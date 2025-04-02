use std::collections::HashMap;
use std::future::Future;

use crate::db::RedisSchema;
use crate::models::{self, ExamMetadata, SectionedQuestions};
use crate::{db::RedisClient, errors::AppError};
use actix_web::dev::{Service, ServiceRequest, ServiceResponse};
use actix_web::{web, HttpRequest, HttpResponse, Result, Scope};
use deadpool_redis::Connection;
use redis::AsyncCommands;
use tokio::task;
use uuid::Uuid;

fn get_short_uuid(length: usize) -> String {
    let uuid = Uuid::new_v4();
    uuid.to_string().chars().take(length).collect::<String>()
}

pub fn exam_routes() -> Scope {
    actix_web::web::scope("/exam")
        .service(web::resource("/create").route(web::post().to(create_exam)))
        .service(web::resource("/{exam_id}").route(web::post().to(get_exam)))
        .service(
            web::resource("/admin/{exam_id}")
                .route(web::get().to(get_exam))
                .wrap_fn(|req, srv| {
                    let fut = srv.call(req);

                    // After the response is completed, add additional logic
                    async {
                        let res = fut.await;

                        if let Ok(response) = &res {
                            println!("Responding with status: {}", response.status());
                        }

                        res
                    }
                }),
        )
}

async fn insert_exam_into_redis(
    redis_client: &RedisClient,
    exam: &models::Exam,
) -> Result<HttpResponse, AppError> {
    let mut conn = redis_client.get_connection().await?;
    let exam_id = get_short_uuid(4);
    let exam_key = RedisSchema::exam_key(&exam_id);

    let mut pipe = redis::pipe();
    pipe.atomic();

    pipe.hset_multiple(
        &exam_key,
        &[
            (RedisSchema::FIELD_NAME, &exam.exam_name),
            (RedisSchema::FIELD_DETAILS, &exam.details),
            (RedisSchema::FIELD_DURATION, &exam.duration.to_string()),
        ],
    );

    for (_, question) in &exam.questions {
        let question_id = get_short_uuid(4);
        let question_key = RedisSchema::question_key(&exam_id, &question_id);

        pipe.hset_multiple(
            &question_key,
            &[
                (RedisSchema::FIELD_NAME, &question.question),
                (
                    RedisSchema::FIELD_OPTIONS,
                    &serde_json::to_string(&question.options).unwrap_or_default(),
                ),
                (RedisSchema::FIELD_MARKS, &question.marks.to_string()),
                (RedisSchema::FIELD_SECTION, &question.section),
            ],
        );
    }

    let _: () = pipe
        .query_async(&mut conn)
        .await
        .map_err(AppError::RedisError)?;

    Ok(HttpResponse::Ok().json(models::ExamId { exam_id }))
}

async fn fetch_exam_from_redis(
    redis_client: &RedisClient,
    exam_id: String,
) -> Result<HttpResponse, AppError> {
    let mut conn = redis_client.get_connection().await?;
    let exam_key = RedisSchema::exam_key(&exam_id);

    let exam_metadata = fetch_exam_metadata(&mut conn, &exam_key).await?;
    let sections = fetch_questions(&mut conn, exam_id).await?;

    let exam = models::ExamResponse {
        exam_name: exam_metadata.exam_name,
        details: exam_metadata.details,
        duration: exam_metadata.duration,
        sections: sections,
    };

    Ok(HttpResponse::Ok().json(exam))
}

async fn fetch_exam_metadata(
    conn: &mut Connection,
    exam_key: &str,
) -> Result<ExamMetadata, AppError> {
    let metadata: HashMap<String, String> = conn.hgetall(exam_key).await?;
    let exam_name = metadata
        .get(RedisSchema::FIELD_NAME)
        .cloned()
        .unwrap_or_default();
    let details = metadata
        .get(RedisSchema::FIELD_DETAILS)
        .cloned()
        .unwrap_or_default();
    let duration = metadata
        .get(RedisSchema::FIELD_DURATION)
        .and_then(|d| d.parse::<u32>().ok())
        .unwrap_or(0);
    Ok(ExamMetadata {
        exam_name,
        details,
        duration,
    })
}

async fn fetch_questions(
    conn: &mut Connection,
    exam_id: String,
) -> Result<SectionedQuestions, AppError> {
    let questions_pattern = RedisSchema::question_key(&exam_id, "*");

    let question_keys: Vec<String> = redis::cmd("KEYS")
        .arg(&questions_pattern)
        .query_async(conn)
        .await
        .map_err(AppError::RedisError)?;

    let mut tasks = Vec::new();
    for key in question_keys {
        let mut conn = conn.clone();
        let exam_id_clone = exam_id.clone();

        tasks.push(task::spawn(async move {
            let metadata: HashMap<String, String> = conn.hgetall(&key).await.unwrap_or_default();

            let question = models::QuestionResponse {
                question: metadata
                    .get(RedisSchema::FIELD_NAME)
                    .cloned()
                    .unwrap_or_default(),
                options: serde_json::from_str(
                    metadata
                        .get(RedisSchema::FIELD_OPTIONS)
                        .unwrap_or(&"[]".to_string()),
                )
                .unwrap_or(vec![]),
                marks: metadata
                    .get(RedisSchema::FIELD_MARKS)
                    .and_then(|m| m.parse().ok())
                    .unwrap_or(0),
                section: metadata
                    .get(RedisSchema::FIELD_SECTION)
                    .cloned()
                    .unwrap_or_default(),
                question_id: key.split(':').last().unwrap_or_default().to_string(),
                exam_id: exam_id_clone.to_string(),
            };

            (question.section.clone(), question)
        }));
    }

    let mut section_map: HashMap<String, Vec<models::QuestionResponse>> = HashMap::new();

    for task in tasks {
        let (section, question) = task.await.unwrap();
        section_map
            .entry(section)
            .or_insert_with(Vec::new)
            .push(question);
    }

    Ok(models::SectionedQuestions {
        sections: section_map,
    })
}

async fn create_exam(
    app_state: web::Data<models::AppState>,
    req_body: web::Json<models::Exam>,
) -> Result<HttpResponse, AppError> {
    insert_exam_into_redis(&app_state.redis_client, &req_body).await
}

async fn get_exam(
    app_state: web::Data<models::AppState>,
    exam_id: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    fetch_exam_from_redis(&app_state.redis_client, exam_id.into_inner()).await
}
