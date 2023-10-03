mod db;
mod routes;

use actix_cors::Cors;
use actix_web::{http, web, App, HttpServer};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use routes::{get_all_tasks, get_task_by_id, status, change_task, create_tast};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let manager = SqliteConnectionManager::file("test.db");
    let pool = Pool::new(manager).expect("Couldn't create pool");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allowed_headers(vec![
                http::header::AUTHORIZATION,
                http::header::ACCEPT,
                http::header::CONTENT_TYPE,
            ])
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
            .service(status)
            .service(get_all_tasks)
            .service(get_task_by_id)
            .service(change_task)
            .service(create_tast)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
    .unwrap();

    Ok(())
}

/*
 * Table task
 * id integer primary key,
 * task text ,
 * task_time text,
 * subject text
*/

#[cfg(test)]
mod tests {
    use rusqlite::Connection;

    use crate::db;

    #[test]
    fn create_db_table() {
        let conn = Connection::open("test.db").unwrap();
        conn.execute(
            "CREATE table task (
                    id integer primary key, 
                    task text, 
                    time text, 
                    subject text
                )",
            [],
        )
        .unwrap();
    }

    #[test]
    fn create_task() {
        use db::Task;
        let conn = Connection::open("test.db").unwrap();
        let new_task = Task::default();
        let res = conn.execute("INSERT INTO task (task, time, subject) VALUES (?1, ?2, ?3)", [&new_task.task.unwrap(), &new_task.time.unwrap(), &new_task.subject.unwrap()]);
        println!("{:?}", res);
    }
}
