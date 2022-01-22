use std::io;
use std::sync::Arc;

#[macro_use]
extern crate diesel;

use actix_multipart::Multipart;
use actix_web::{
    dev::ServiceRequest, error, middleware::Logger, web, App, Error, HttpResponse, HttpServer,
};
use actix_web_httpauth::extractors::basic::BasicAuth;
use actix_web_httpauth::middleware::HttpAuthentication;
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;
use serde::{Deserialize, Serialize};

mod bank;
mod db;
mod graphql_schema;
mod nordnet;
mod schema;

use bank::statement_handler;
use db::PgPool;
use graphql_schema::{create_schema, Schema};

#[actix_web::main]
async fn main() -> io::Result<()> {
    // Set up logger middleware
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    // Create a pool of Postgres connections
    let pool: PgPool = db::create_pool();

    // Create Juniper schema
    let schema: Arc<Schema> = Arc::new(create_schema());

    // Start http server
    HttpServer::new(move || {
        let auth_middleware = HttpAuthentication::basic(auth_validator);
        App::new()
            .wrap(Logger::default())
            .wrap(auth_middleware)
            .data(pool.clone())
            .data(schema.clone())
            .route("/api/login", web::post().to(login))
            .route("/api/statement", web::post().to(upload_statement))
            .route("/api/graphql", web::post().to(graphql))
            .route("/graphiql", web::get().to(graphiql))
    })
    .bind("127.0.0.1:8081")?
    .run()
    .await
}

async fn auth_validator(
    req: ServiceRequest,
    credentials: BasicAuth,
) -> Result<ServiceRequest, Error> {
    let username = credentials.user_id();
    let password = credentials.password();

    match password {
        None => Err(error::ErrorBadRequest("Missing password".to_string())),
        Some(password) => {
            if username == "admin" && password == "admin" {
                Ok(req)
            } else {
                Err(error::ErrorUnauthorized("Invalid credentials".to_string()))
            }
        }
    }
}

async fn login() -> Result<HttpResponse, Error> {
    // Credentials are actually handled by auth middleware, so this is just a dummy endpoint.
    Ok(HttpResponse::Ok().body("Login successful"))
}

#[derive(Debug, Serialize, Deserialize)]
struct UploadParams {
    account_id: i32,
}

async fn upload_statement(
    params: web::Query<UploadParams>,
    payload: Multipart,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, Error> {
    let upload_status = statement_handler::read_statement(params.account_id, payload, &pool).await;

    match upload_status {
        Ok(_) => Ok(HttpResponse::Ok().body("Statement uploaded successfully")),
        Err(err) => Err(error::ErrorInternalServerError(err)),
    }
}

async fn graphql(
    st: web::Data<Arc<Schema>>,
    data: web::Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    let body = web::block(move || {
        let res = data.execute_sync(&st, &());
        serde_json::to_string(&res)
    })
    .await?;

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(body))
}

async fn graphiql() -> HttpResponse {
    let html = graphiql_source("http://localhost:8081/api/graphql", None);

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}
