use actix_web::{get, web, HttpResponse, Responder};
use reqwest::{header::HeaderMap, Client};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::config::{self, Config};

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
            Ok(HttpResponse::Ok().body(text))
        }
        Err(err) => {
            eprintln!("Error sending GraphQL request: {:?}", err);
            Ok(HttpResponse::BadRequest().finish())
        }
    }
}
