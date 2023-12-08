pub(crate) mod routes;

use crate::routes::auth;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(auth::authenticate_user),
    components(schemas(auth::AuthenticateUserParams, auth::AuthenticatedTokenResult,))
)]
struct ApiDoc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let openapi = ApiDoc::openapi();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(SwaggerUi::new("/docs/{_:.*}").url("/api-docs/openapi.json", openapi.clone()))
            .service(auth::authenticate_user)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
