#[cfg(all(feature = "axum", not(target_arch = "wasm32")))]
pub mod axum {
    use std::sync::Arc;

    use axum::extract::State;
    use axum::http::StatusCode;
    use axum::response::IntoResponse;
    use axum::Json;

    use crate::{LinkRequest, LinkResponse, Resolver};

    /// Axum handler that resolves [`LinkRequest`]s.
    ///
    /// ```ignore
    /// let resolver = Arc::new(
    ///     Resolver::new()
    ///         .register::<Post>(|id| async move { db::get_post(id).await })
    /// );
    ///
    /// let app = axum::Router::new()
    ///     .route("/api/link", axum::routing::post(linked_state_handler))
    ///     .with_state(resolver);
    /// ```
    pub async fn linked_state_handler(
        State(resolver): State<Arc<Resolver>>,
        Json(req): Json<LinkRequest>,
    ) -> impl IntoResponse {
        match resolver.resolve_request(&req).await {
            Ok(val) => (
                StatusCode::OK,
                Json(LinkResponse {
                    ok: Some(val),
                    error: None,
                }),
            ),
            Err(err_val) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(LinkResponse {
                    ok: None,
                    error: Some(err_val),
                }),
            ),
        }
    }
}

#[cfg(all(feature = "actix", not(target_arch = "wasm32")))]
pub mod service {
    use actix_web::web::{Data, Json};
    use actix_web::HttpResponse;

    use crate::{LinkRequest, LinkResponse, Resolver};

    /// Actix handler that resolves [`LinkRequest`]s.
    ///
    /// ```ignore
    /// let resolver = Data::new(
    ///    Resolver::new()
    ///        .register::<Post>(|id| async move { db::get_post(id).await })
    /// )
    ///
    /// HttpServer::new(move || {
    ///    App::new()
    ///        .route("/api/link", post().to(linked_state_handler))
    ///        .data(resolver)
    /// })
    ///    .bind(("0.0.0.0", 8080))?
    ///    .run()
    ///    .await
    /// ```
    pub async fn linked_state_handler(
        resolver: Data<Resolver>,
        Json(req): Json<LinkRequest>,
    ) -> HttpResponse {
        match resolver.resolve_request(&req).await {
            Ok(val) => HttpResponse::Ok().json(LinkResponse {
                ok: Some(val),
                error: None,
            }),

            Err(err_val) => HttpResponse::UnprocessableEntity().json(LinkResponse {
                ok: None,
                error: Some(err_val),
            }),
        }
    }
}
