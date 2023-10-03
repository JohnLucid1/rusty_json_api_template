use actix_web::{error, web, Error};
use serde::{Deserialize, Serialize};

pub type Pool = r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>;
pub type Connection = r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>;

#[allow(clippy::enum_variant_names)]
pub enum Queries {
    GetAllTasks,
    TaskById(u32),
    ChangeTask(Task),
    NewTask(Task),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: u32,
    pub task: Option<String>,
    pub time: Option<String>,
    pub subject: Option<String>,
}

impl Default for Task {
    fn default() -> Self {
        Self {
            id: 0,
            task: Some("Do my homework".into()),
            time: Some("50 min".into()),
            subject: Some("math".into()),
        }
    }
}

pub fn get_tasks(conn: &Connection) -> Result<Vec<Task>, rusqlite::Error> {
    let mut stmt = conn.prepare("SELECT id, task, time, subject FROM task")?;
    let tasks = stmt
        .query_map([], |row| {
            Ok(Task {
                id: row.get(0).unwrap(),
                task: row.get(1).unwrap(),
                time: row.get(2).unwrap(),
                subject: row.get(3).unwrap(),
            })
        })
        .unwrap();

    let tasks = tasks.map(|task| task.unwrap()).collect();
    Ok(tasks)
}

pub fn get_task_by_id(conn: &Connection, id: u32) -> Result<Vec<Task>, rusqlite::Error> {
    let mut stmt = conn.prepare("SELECT id, task, time, subject FROM task WHERE id = (?)")?;
    let tasks = stmt
        .query_map([&id], |row| {
            Ok(Task {
                id: row.get(0).unwrap(),
                task: row.get(1).unwrap(),
                time: row.get(2).unwrap(),
                subject: row.get(3).unwrap(),
            })
        })
        .unwrap();

    let tasks = tasks.map(|task| task.unwrap()).collect();
    Ok(tasks)
}

pub fn change_task(conn: &Connection, task: Task) -> Result<Vec<Task>, rusqlite::Error> {
    let query = "UPDATE task set task = (?1), time = (?2), subject = (?3) WHERE id = (?4)";
    let new_task = task.clone();
    match conn.execute(
        query,
        [
            &task.task.unwrap(),
            &task.time.unwrap(),
            &task.subject.unwrap(),
            &task.id.to_string(),
        ],
    ) {
        Ok(_) => Ok(vec![new_task]),
        Err(err) => Err(err),
    }
}

pub fn new_task(conn: &Connection,  task: Task) -> Result<Vec<Task>, rusqlite::Error> {
    let query = "INSERT INTO task (task, time, subject) VALUES (?1, ?2, ?3)";
    let mut  new_task = task.clone();
    match conn.execute(
        query,
        [
            &task.task.unwrap(),
            &task.time.unwrap(),
            &task.subject.unwrap(),
        ],
    ) {
        Ok(_) => {
            let last_id = conn.last_insert_rowid();
            new_task = Task  {
                id: last_id as u32, 
                ..new_task
            };
            
            Ok(vec![new_task])
        }
        Err(err) => Err(err),
    }
}

pub async fn execute(pool: &Pool, query: Queries) -> Result<Vec<Task>, Error> {
    let pool = pool.clone();

    let conn = web::block(move || pool.get())
        .await?
        .map_err(error::ErrorInternalServerError)?;

    web::block(move || match query {
        Queries::TaskById(id) => get_task_by_id(&conn, id),
        Queries::GetAllTasks => get_tasks(&conn),
        Queries::ChangeTask(task) => change_task(&conn, task),
        Queries::NewTask(task) => new_task(&conn, task),
    })
    .await?
    .map_err(error::ErrorInternalServerError)
}
