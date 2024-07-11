use yew::{function_component, html, Html};

#[function_component(App)]
pub fn app() -> Html {
    use wasm_bindgen_futures::spawn_local;
    let src = gloo_net::eventsource::futures::EventSource::new("//localhost:9000/sse");
    use futures::StreamExt;

    log::info!("{src:?}");

    if let Ok(mut event_source) = src {
        let mut stream1 = event_source.subscribe("test-event").unwrap();

        spawn_local(async move {
            log::info!("{event_source:?}");
            log::info!("First: {:?}", stream1.next().await);
            log::info!("{event_source:?}");
            while let Some(Ok((event_type, msg))) = stream1.next().await {
                log::warn!("New message: {event_type} {msg:?}");
            }
            log::info!("Finished");
        });
        log::info!("After spawn");
    }

    html!()
}

fn main() {
    dotenv::dotenv().ok();

    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
