use actix_cors::Cors;
use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use log::{debug, info};
use std::env;
use std::time::SystemTime;

mod errors;
mod slack;

use slack::*;

fn init_logger() {
    const DEFAULT_LOG: &str = "actix_web=error,tile_slack=info";

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
async fn handle_slack(from_slack: web::Form<SlackReceivedForm>) -> impl Responder {
    debug!("{:?}", from_slack);

    let timestamp = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    };

    let to_slack = SlackCommandResponse {
        response_type: "in_channel".to_string(),
        attachments: vec![Attachment {
            author_name: from_slack.user_name.clone(),
            fallback: format!("\"{}\"", from_slack.text),
            color: "#36a64f".to_string(),
            image_url: "".to_string(),
            ts: timestamp,
        }],
    };
    HttpResponse::Ok().json(to_slack)
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

        App::new().wrap(cors).service(handle_slack)
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

        let form_data = Form(SlackReceivedForm {
            token: "mysecrettoken".to_string(),
            text: "my wisdown for the tile".to_string(),
            user_name: "owarnier".to_string(),
            team_id: "T0001".to_string(),
            team_domain: "example".to_string(),
            enterprise_id: "E0001".to_string(),
            enterprise_name: "Globular%20Construct%20Inc".to_string(),
            channel_id: "2147483705".to_string(),
            channel_name: "test".to_string(),
            user_id: "U2147483697".to_string(),
            command: "/tegeltje".to_string(),
            response_url: "https://hooks.slack.com/commands/1234/5678".to_string(),
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
