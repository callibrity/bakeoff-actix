#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use actix_web::{middleware, web, App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

// We define a custom type for connection pool to use later.
pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

mod handlers;
mod models;
mod schema;


pub struct DbConn(diesel::PgConnection);

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    //load .env into environment variable
    dotenv::dotenv().ok();
    

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("ERROR"));
    //set up db connection pool
    let db_host = std::env::var("DB_HOST").expect("DB_HOST");
    let db_port = std::env::var("DB_PORT").expect("DB_PORT");
    let db_name = std::env::var("DB_NAME").expect("DB_NAME");
    let db_user = std::env::var("DB_USER").expect("DB_USER");
    let db_pass = std::env::var("DB_PASS").expect("DB_PASS");

    let database_url = std: format!("postgres://{}:{}@{}:{}/{}", db_user, db_pass, db_host, db_port, db_name);

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: DbPool = r2d2::Pool::builder()
        .max_size(20)
        .build(manager)
        .expect("Failed to create pool.");

    {    //create a scope so conn is freed
       let conn = &mut pool.clone().get().unwrap();
        embed_migrations!();
        embedded_migrations::run(conn).expect("failed migration");
    }    

        let port = std::env::var("PORT").expect("PORT");

    
        HttpServer::new(move || {
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .wrap(middleware::Logger::default())
                .route("/", web::get().to(|| async { "Actix REST API" }))
                .service(handlers::index)
                .service(handlers::create)
                .service(handlers::show)
                .service(handlers::update)
                .service(handlers::destroy)
        })
        .bind(("0.0.0.0", port.parse::<u16>().unwrap()))?
        .run()
        .await
}
