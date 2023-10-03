use actix_web::{get, post, put, web, Error as AWError, HttpResponse};
use serde::{Deserialize, Serialize};

use crate::db::{self, Pool, Queries, Task};

#[get("/status")]
pub async fn status() -> HttpResponse {
    HttpResponse::Ok().body("Server is runnning")
}

#[get("/tasks")]
pub async fn get_all_tasks(db: web::Data<Pool>) -> Result<HttpResponse, AWError> {
    let res = db::execute(&db, Queries::GetAllTasks).await?;
    Ok(HttpResponse::Ok().json(res))
}

#[get("/tasks/{id}")]
pub async fn get_task_by_id(
    path: web::Path<u32>,
    db: web::Data<Pool>,
) -> Result<HttpResponse, AWError> {
    let id: u32 = path.into_inner();
    let res = db::execute(&db, Queries::TaskById(id)).await?;
    Ok(HttpResponse::Ok().json(res))
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TaskReq {
    pub task: Option<String>,
    pub time: Option<String>,
    pub subject: Option<String>,
}
#[post("/tasks/")]
pub async fn create_tast(
    db: web::Data<Pool>,
    task_req: web::Json<TaskReq>,
) -> Result<HttpResponse, AWError> {
    let task_req = task_req.into_inner();
    let new_task = Task {
        id: 0,
        task: task_req.task,
        time: task_req.time,
        subject: task_req.subject,
    };

    let res = db::execute(&db, Queries::NewTask(new_task)).await?;
    Ok(HttpResponse::Ok().json(res))
}

#[put("/tasks/")]
pub async fn change_task(
    db: web::Data<Pool>,
    new_task: web::Json<Task>,
) -> Result<HttpResponse, AWError> {
    let res = db::execute(&db, Queries::ChangeTask(new_task.into_inner())).await?;
    Ok(HttpResponse::Ok().json(res))
}
