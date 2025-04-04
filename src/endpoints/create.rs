use crate::db;
use crate::entities::{correct_option, details, exam, options, questions, sections};
use crate::errors::AppError;
use crate::models;
use actix_web::{web, HttpResponse};
use rand::Rng;
use sea_orm::EntityTrait;
use sea_orm::TransactionTrait;
use sea_orm::{ActiveModelTrait, Set};

/// Generates a random integer of a specified number of digits.
///
/// This function generates a random integer in the range defined by the number of digits
/// specified. For example, if the size is 2, it will generate a number between 10 and 99.
///
/// # Arguments
///
/// * `size` - The number of digits the generated integer should have.
///
/// # Returns
///
/// Returns a random integer within the range defined by the number of digits specified.
///
/// # Example
///
/// ```rust
/// let rand_num = get_random_int(3);
/// println!("Random number: {}", rand_num);
/// ```
fn get_random_int(size: usize) -> i32 {
    let min = 10_i32.pow((size - 1) as u32);
    let max = 10_i32.pow(size as u32) - 1;
    let mut rng = rand::thread_rng();
    rng.gen_range(min..=max)
}

/// Inserts a new exam and its associated details, sections, questions, options, and correct options
/// into the database within a transaction.
///
/// The `insert` function first starts a database transaction, then proceeds to insert various elements
/// related to an exam, including exam details, sections, questions, options, and correct options.
/// The function will insert them into the respective tables using `insert_many` for bulk inserts.
///
/// If any of the database operations fail, the transaction will be rolled back, ensuring atomicity.
/// If everything is successful, the transaction is committed and an `ExamId` response is returned.
///
/// # Arguments
///
/// * `db_client` - A reference to the database client to interact with the database.
/// * `exam` - A reference to the `models::Exam` object containing the details of the exam.
///
/// # Returns
///
/// Returns an `HttpResponse` containing the `ExamId` if the insertion is successful.
/// Otherwise, it returns an `AppError` if any error occurs during the insertion process.
///
/// # Example
///
/// ```rust
/// let result = insert(&db_client, &exam).await;
/// match result {
///     Ok(response) => println!("Exam created with ID: {}", response.examine_id),
///     Err(e) => println!("Error inserting exam: {}", e),
/// }
/// ```
async fn insert(db_client: &db::DbClient, exam: &models::Exam) -> Result<HttpResponse, AppError> {
    // Begin a new transaction
    let txn = db_client.db.begin().await?;
    let exam_id = get_random_int(5);

    exam::ActiveModel {
        id: Set(exam_id),
        ..Default::default()
    }
    .insert(&txn)
    .await?;

    // Insert exam first
    let exam_details = details::ActiveModel {
        id: Set(get_random_int(5)),
        exam_id: Set(exam_id),
        title: Set(exam.exam_description.title.clone()),
        description: Set(Some(exam.exam_description.description.clone())),
        duration: Set(exam.exam_description.duration),
        passing_score: Set(exam.exam_description.passing_score),
        ..Default::default()
    }
    .insert(&txn)
    .await?;

    // Insert sections
    let sections: Vec<sections::ActiveModel> = exam
        .sections
        .iter()
        .map(|sec| {
            sections::ActiveModel {
                id: Set(sec.id.clone()),
                details_id: Set(exam_details.id.clone()), // Set the foreign key to the correct details_id
                title: Set(sec.title.clone()),
                ..Default::default()
            }
        })
        .collect();
    sections::Entity::insert_many(sections).exec(&txn).await?;

    // Insert questions
    let questions: Vec<questions::ActiveModel> = exam
        .questions
        .iter()
        .map(|q| questions::ActiveModel {
            id: Set(q.id.clone()),
            section_id: Set(q.section_id.clone()),
            text: Set(q.text.clone()),
            description: Set(Some(q.description.clone())),
            marks: Set(q.marks),
            ..Default::default()
        })
        .collect();
    questions::Entity::insert_many(questions).exec(&txn).await?;

    // Insert options
    let options: Vec<options::ActiveModel> = exam
        .options
        .iter()
        .map(|opt| options::ActiveModel {
            id: Set(opt.id.clone()),

            question_id: Set(opt.question_id.clone()),
            text: Set(opt.text.clone()),
            is_correct: Set(Some(opt.is_correct)),
            ..Default::default()
        })
        .collect();

    options::Entity::insert_many(options).exec(&txn).await?;

    // Insert current options
    let options: Vec<correct_option::ActiveModel> = exam
        .options
        .iter()
        .filter(|opt| opt.is_correct)
        .map(|opt| correct_option::ActiveModel {
            option_id: Set(opt.id),
            ..Default::default()
        })
        .collect();

    correct_option::Entity::insert_many(options)
        .exec(&txn)
        .await?;
    // Commit the transaction if all inserts are successful
    txn.commit().await?;

    Ok(HttpResponse::Ok().json(models::ExamId { exam_id }))
}

/// Handles the creation of a new exam by calling the `insert` function to save the exam
/// and its related data into the database.
///
/// This function is an Actix-web handler for creating a new exam. It expects a `web::Json`
/// body containing an `Exam` object. After receiving the request, it calls the `insert` function
/// to handle the database insertion process. If successful, it returns an HTTP `200 OK` response
/// with the generated `exam_id`. If an error occurs during insertion, an error response is returned.
///
/// # Arguments
///
/// * `app_state` - An Actix-web `Data` containing the app's state, which includes the database client.
/// * `req_body` - A JSON body containing the exam details to be inserted.
///
/// # Returns
///
/// Returns an `HttpResponse` containing the `ExamId` if the exam is successfully created, or an error if any issue occurs.
///
/// # Example
///
/// ```rust
/// let response = create_exam(app_state, req_body).await;
/// match response {
///     Ok(resp) => println!("Exam created with ID: {}", resp.exam_id),
///     Err(e) => println!("Error creating exam: {}", e),
/// }
/// ```
pub async fn create_exam(
    app_state: web::Data<models::AppState>,
    req_body: web::Json<models::Exam>,
) -> Result<HttpResponse, AppError> {
    insert(&app_state.db_client, &req_body).await
}
