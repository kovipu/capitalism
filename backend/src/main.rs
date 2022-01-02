use std::io;
use std::sync::Arc;

use actix_multipart::Multipart;
use actix_web::{dev::ServiceRequest, error, web, App, Error, HttpResponse, HttpServer, middleware::Logger};
use actix_web_httpauth::extractors::basic::BasicAuth;
use actix_web_httpauth::middleware::HttpAuthentication;
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;
use log;

mod bank;
mod nordnet;
mod db;

use bank::statement;
use nordnet::schema::{create_schema, Schema};

#[actix_web::main]
async fn main() -> io::Result<()> {
    // Set up logger middleware
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    // Create a pool of Postgres connections
    let pool = db::establish_connection();

    // Create Juniper schema
    let schema = Arc::new(create_schema());

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
            .route("/api/graphiql", web::get().to(graphiql))
            .route("/api/graphql", web::post().to(graphql))
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

async fn upload_statement(payload: Multipart) -> Result<HttpResponse, Error> {
    let upload_status = statement::read_statement(payload).await;

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
    let html = graphiql_source("http://localhost:8081/graphql", None);

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}
