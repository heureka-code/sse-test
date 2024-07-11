use actix_web::{get, Responder};
use actix_web_lab::sse;
use std::time::Duration;
use tokio::time::sleep;

#[get("/sse")]
async fn stream_updates() -> impl Responder {
    let (tx, rx) = tokio::sync::mpsc::channel(10);

    tokio::spawn(async move {
        for i in 0..10 {
            let _ = tx
                .send(sse::Data::new(i.to_string()).event("test-event").into())
                .await;
            sleep(tokio::time::Duration::from_secs(1)).await;
        }
    });

    let sse = sse::Sse::from_infallible_receiver(rx).with_retry_duration(Duration::from_secs(10));
    let sse = sse
        .customize()
        .append_header(("access-control-allow-origin", "*"))
        .append_header(("access-control-allow-headers", "content-type"));
    return sse;
}
