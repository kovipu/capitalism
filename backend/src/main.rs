use std::io;
use std::sync::Arc;

use actix_web::{web, App, Error, HttpResponse, HttpServer};
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;

mod nordnet;
use nordnet::schema::{create_schema, Schema};

#[actix_web::main]
async fn main() -> io::Result<()> {
    // Create Juniper schema
    let schema = Arc::new(create_schema());

    // Start http server
    HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .route("/graphiql", actix_web::web::get().to(graphiql))
            .route("/graphql", actix_web::web::post().to(graphql))
    })
    .bind("127.0.0.1:8081")?
    .run()
    .await
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
