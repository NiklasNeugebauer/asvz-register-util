extern crate hyper;
extern crate tokio;

use hyper_tls::HttpsConnector;
use hyper::{Client, Body, Method, Request};
use hyper::header::AUTHORIZATION;

use futures::{join, executor};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let request1 = check_available("LESSON", "Bearer TOKEN");
    let res = request1.await;

    Ok(())
} 

async fn request_enrollment(lesson_id: &str, auth_token: &str) -> Result<(), Box<dyn std::error::Error>> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let uri = format!("https://schalter.asvz.ch/tn-api/api/Lessons/{}/enroll??t=0", lesson_id);

    loop {
        let req = Request::builder()
        .method(Method::POST)
        .uri(&uri)
        .header(AUTHORIZATION, auth_token)
        .header("content-length", 0)
        .body(Body::empty())?;

        let resp = client.request(req).await?;
        let status = resp.status();
        if status.is_success() {
            match resp.headers().get("placeNumber") {
                Some(placement) => println!("Got Place Number: {}", placement.to_str().unwrap()),
                None => println!("Success, but no placement. Another thread may have enrolled. Stopping!"),
            };
            
            //break;
        }
        else {
            println!("Status: {}", status);
        }
    }

    Ok(())
}

async fn check_available(lesson_id: &str, auth_token: &str) -> Result<(), Box<dyn std::error::Error>> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let uri = format!("https://schalter.asvz.ch/tn-api/api/Lessons/{}??t=0", lesson_id);
    let req = Request::builder()
        .method(Method::GET)
        .uri(uri)
        .header(AUTHORIZATION, auth_token)
        .body(Body::empty())?;

    let resp = client.request(req).await?;

    println!("Response: {}", resp.status());

    Ok(())
}