use actix_middleware_discord_interactions::DiscordInteractions;
use actix_web::{web, App, HttpResponse, HttpServer};
use std::env;

// For testing during development
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    const PORT: u16 = 3000;
    let public_key = env::var("PUBLIC_KEY")
        .unwrap_or_else(|_| panic!("environment variable \"PUBLIC_KEY\" not found!"));

    HttpServer::new(move || {
        App::new()
            .wrap(DiscordInteractions {
                public_key: public_key.to_string(),
            })
            .route("/", web::post().to(HttpResponse::Ok))
    })
    .bind(("127.0.0.1", PORT))?
    .run()
    .await
}
