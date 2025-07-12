use actix_cors::Cors;
use actix_web::{http::header, web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use reqwest::Client as HttpClient;
use async_trait::async_trait;
use std::sync::Mutex;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ForexPair {
    base: String,
    quote: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ForexPrice {
    base: String,
    quote: String,
    price: f64,
    source: String,
}

#[derive(Clone)]
struct AppState {
    client: HttpClient,
}

#[derive(Deserialize)]
struct ForexPairsRequest {
    pairs: Vec<ForexPair>,
}

async fn fetch_forex_price_exchangerate_host(client: &HttpClient, base: &str, quote: &str) -> Option<f64> {
    let url = format!("https://api.exchangerate.host/latest?base={}&symbols={}", base, quote);
    let resp = client.get(&url).send().await.ok()?;
    let json: serde_json::Value = resp.json().await.ok()?;
    json["rates"][quote].as_f64()
}

async fn fetch_forex_price_open_er_api(client: &HttpClient, base: &str, quote: &str) -> Option<f64> {
    let url = format!("https://open.er-api.com/v6/latest/{}", base);
    let resp = client.get(&url).send().await.ok()?;
    let json: serde_json::Value = resp.json().await.ok()?;
    json["rates"][quote].as_f64()
}

async fn fetch_forex_price(client: &HttpClient, base: &str, quote: &str) -> Option<(f64, String)> {
    if let Some(price) = fetch_forex_price_exchangerate_host(client, base, quote).await {
        return Some((price, "exchangerate.host".to_string()));
    }
    if let Some(price) = fetch_forex_price_open_er_api(client, base, quote).await {
        return Some((price, "open.er-api.com".to_string()));
    }
    None
}

async fn get_forex_prices(
    app_state: web::Data<AppState>,
    req: web::Json<ForexPairsRequest>,
) -> impl Responder {
    let mut results = Vec::new();
    for pair in &req.pairs {
        if let Some((price, source)) = fetch_forex_price(&app_state.client, &pair.base, &pair.quote).await {
            results.push(ForexPrice {
                base: pair.base.clone(),
                quote: pair.quote.clone(),
                price,
                source,
            });
        }
    }
    HttpResponse::Ok().json(results)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client = HttpClient::new();
    let data = web::Data::new(AppState { client });

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::permissive()
                    .allowed_origin_fn(|origin, _req_head| {
                        origin.as_bytes().starts_with(b"http://localhost") || origin == "null"
                    })
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600)
            )
            .app_data(data.clone())
            .route("/forex-prices", web::post().to(get_forex_prices))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}