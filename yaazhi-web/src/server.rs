use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use tracing::{error, info};
use tracing_subscriber;
use yaazhi_runtime::xml::config::yaazhi_config::YaazhiConfig;

/// Start the Yaazhi Web Server
pub async fn start_server(config: YaazhiConfig) -> Result<(), Box<dyn std::error::Error>> {

    // Initialize tracing (logging)
    tracing_subscriber::fmt::init();
   
    let web_config = &config.web_server;
    let bind_address = web_config
        .bind_address()
        .ok_or("Invalid host or port in config")?;

    info!("🌐 Starting Yaazhi Web Server at {}", bind_address);

    HttpServer::new(move || {
        let  app = App::new()
        .route("/health", web::get().to(health_check))
        .route("/", web::get().to(hello));

        app
    })
    .bind(&bind_address)?
    .run()
    .await?;

    Ok(())
}

/// Health-check endpoint

async fn hello() -> impl Responder {
    HttpResponse::Ok().body("<h1>Hello, Yaazhi!</h1>")
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok().body(format!("<h1 style='{}'>Yaazhi Web Server is running! <span>🌟</span></h1>", "color:red"))
}

