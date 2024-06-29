use std::{collections::BTreeSet, sync::Arc};

use actix_web::http;

pub(crate) fn get_cors(allowed_origin_suffixes: Arc<BTreeSet<String>>) -> actix_cors::Cors {
    actix_cors::Cors::default()
        .allowed_origin_fn(move |origin, _req_head| {
            let org = origin.as_bytes();
            let allowed = allowed_origin_suffixes
                .iter()
                .any(|origin| org.ends_with(origin.as_bytes()));
            log::debug!("(CORS check) tried origin is: {origin:?}, allowed={allowed}");
            allowed
        })
        .allowed_origin("http://localhost:9000")
        .allowed_origin("http://localhost:8080")
        .allowed_methods(vec!["GET", "POST", "OPTIONS", "PUT", "HEAD", "CONNECT"])
        .allowed_headers(vec![
            http::header::AUTHORIZATION,
            http::header::ACCEPT,
            http::header::CONTENT_TYPE,
        ])
        .max_age(3600)
        .allow_any_header()
        .allow_any_method()
}
pub(crate) fn read_allowed_suffixes() -> Arc<BTreeSet<String>> {
    let allowed_origin_suffixes =
        std::env::var(crate::env_keys::ALLOWED_ORIGIN_SUFFIXES).unwrap_or("".into());
    let allowed_origin_suffixes = allowed_origin_suffixes
        .split('|')
        .filter_map(|s| (s.trim() != "").then_some(s.to_owned()))
        .collect::<BTreeSet<_>>();
    std::sync::Arc::new(allowed_origin_suffixes)
}
