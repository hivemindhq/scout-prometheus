use actix_web::{get, web, HttpResponse, Responder};
use reqwest::{header::HeaderMap, Client};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::config::Config;
use crate::types::Team;

#[derive(Serialize, Deserialize, Debug)]
pub struct QueryRequest {
    year: String,
    id: String,
}

pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("hello world!!")
}

#[get("/query/{year}/{id}")]
pub async fn query(
    data: web::Data<Config>,
    body: web::Path<QueryRequest>,
) -> Result<impl Responder, Box<dyn std::error::Error>> {
    let client = Client::new();

    let query = format!(
        r#"
    query {{
        eventByCode(code: "{}", season: {}) {{
          teams {{
            team {{
              number
              name
              events(season: {}) {{
                eventCode
                stats {{
                  ... on {} {{
                    wins
                    losses
                    rank
                    average {{
                      autoPoints
                      totalPoints
                    }}
                  }}
                }}
              }}
            }}
          }}
        }}
      }}
    "#,
        body.id, body.year, body.year, data.season_gql_id
    );

    let request = format!(
        "https://api.ftcscout.org/rest/v1/events/{}/{}/teams",
        body.year, body.id
    );

    println!("{}", request);

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());

    let response = client
        .post(format!("{}", data.gql_api))
        .headers(headers)
        .json(&json!({
          "query": query.to_string()
        }))
        .send()
        .await;

    match response {
        Ok(resp) => {
            let text = resp.text().await.unwrap();

            let text: Vec<Team> = serde_json::from_str(&text).unwrap();

            for team in text.into_iter() {
                println!("{:?}", team)
            }

            Ok(HttpResponse::Ok().body("hiiiii"))
        }
        Err(err) => {
            eprintln!("Error sending GraphQL request: {:?}", err);
            Ok(HttpResponse::BadRequest().body(format!("{}", err)))
        }
    }
}
