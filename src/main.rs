use actix_cors::Cors;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use futures::{future::ok, stream::once};
use log::{debug, info};
use std::env;
use std::time::SystemTime;

mod errors;
mod slack;
mod tile;

use slack::*;
use tile::TileError;

fn init_logger() {
    const DEFAULT_LOG: &str = "actix_web=error,tile_slack=debug";

    let log_setting = match std::env::var("RUST_LOG") {
        Ok(ls) => ls,
        Err(_e) => {
            std::env::set_var("RUST_LOG", DEFAULT_LOG);
            DEFAULT_LOG.to_string()
        }
    };
    env_logger::init();
    debug!("Log set to {}", log_setting);
}

#[post("/slack")]
async fn handle_slack(from_slack: web::Form<SlackReceivedCommand>) -> impl Responder {
    debug!("{:?}", from_slack);

    let timestamp = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    };

    let to_slack = SlackCommandResponse {
        response_type: "in_channel".to_string(),
        attachments: vec![Attachment {
            author_name: from_slack.user_name.clone(),
            fallback: format!("{}", from_slack.text),
            color: "#36a64f".to_string(),
            image_url:
                "https://www.tegeltjes.com/Files/3/8000/8404/ProductPhotos/Large/1052079945.jpg"
                    .to_string(),
            ts: timestamp,
        }],
    };
    HttpResponse::Ok().json(to_slack)
}

#[get("/tegeltje")]
async fn create_tile_image() -> impl Responder {
    let body = tile::create_tile_image("hallo wereld".to_string())
        .await
        .expect("Cannot create Tile Image");
    HttpResponse::Ok().content_type("image/jpg").body(body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_logger();

    //NOTE: PORT is required by HEROKU, so fail when it is not here
    let port = env::var("PORT").expect("Please provide PORT environment variable");
    let bind = format!("0.0.0.0:{}", port);
    info!("Starting tile-slack on port {}", port);
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_header()
            .allow_any_method()
            .supports_credentials()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .service(handle_slack)
            .service(create_tile_image)
    })
    .bind(bind)?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::dev::Service;
    use actix_web::web::Form;
    use actix_web::{http, test, App, Error};

    #[actix_rt::test]
    async fn test_call_slack_ok() -> Result<(), Error> {
        let app = test::init_service(App::new().service(handle_slack)).await;

        let form_data = Form(SlackReceivedCommand {
            token: "mysecrettoken".to_string(),
            text: "my wisdown for the tile".to_string(),
            user_name: "owarnier".to_string(),
            team_id: "T0001".to_string(),
            team_domain: "example".to_string(),
            enterprise_id: Some("E0001".to_string()),
            enterprise_name: Some("Globular%20Construct%20Inc".to_string()),
            channel_id: "2147483705".to_string(),
            channel_name: "test".to_string(),
            user_id: "U2147483697".to_string(),
            command: "/tegeltje".to_string(),
            response_url: "https://hooks.slack.com/commands/1234/5678".to_string(),
            api_app_id: "myappid".to_string(),
            trigger_id: "triggerid".to_string(),
        });

        let req = test::TestRequest::post()
            .uri("/slack")
            .set_form(&form_data)
            .to_request();
        let resp = app.call(req).await.unwrap();
        assert_eq!(resp.status(), http::StatusCode::OK);

        Ok(())
    }
}
