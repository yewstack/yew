#[cfg(feature = "axum")]
pub mod axum {
    use std::sync::Arc;

    use axum::Json;
    use axum::extract::State;
    use axum::http::StatusCode;
    use axum::response::IntoResponse;

    use crate::{LinkRequest, LinkResponse, Resolver};

    /// Axum handler that resolves [`LinkRequest`]s.
    ///
    /// ```
    /// use std::sync::Arc;
    ///
    /// use serde::{Deserialize, Serialize};
    /// use yew_link::axum::linked_state_handler;
    /// use yew_link::{LinkedState, Never, Resolver};
    ///
    /// #[derive(Clone, Debug, Serialize, Deserialize)]
    /// struct Post {
    ///     title: String,
    /// }
    ///
    /// impl LinkedState for Post {
    ///     type Error = Never;
    ///     type Input = u32;
    ///
    ///     const TYPE_KEY: &'static str = "Post";
    /// }
    ///
    /// async fn get_post(_id: u32) -> Result<Post, Never> {
    ///     Ok(Post {
    ///         title: String::new(),
    ///     })
    /// }
    ///
    /// let resolver = Arc::new(Resolver::new().register::<Post, _, _>(|id| get_post(id)));
    ///
    /// let app: axum::Router = axum::Router::new().route(
    ///     "/api/link",
    ///     axum::routing::post(linked_state_handler).with_state(resolver),
    /// );
    /// # let _ = app;
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

#[cfg(feature = "actix")]
pub mod actix {
    use actix_web::HttpResponse;
    use actix_web::web::{Data, Json};

    use crate::{LinkRequest, LinkResponse, Resolver};

    /// Actix handler that resolves [`LinkRequest`]s.
    ///
    /// ```no_run
    /// use actix_web::web::{Data, post};
    /// use actix_web::{App, HttpServer};
    /// use serde::{Deserialize, Serialize};
    /// use yew_link::actix::linked_state_handler;
    /// use yew_link::{LinkedState, Never, Resolver};
    ///
    /// #[derive(Clone, Debug, Serialize, Deserialize)]
    /// struct Post {
    ///     title: String,
    /// }
    ///
    /// impl LinkedState for Post {
    ///     type Error = Never;
    ///     type Input = u32;
    ///
    ///     const TYPE_KEY: &'static str = "Post";
    /// }
    ///
    /// async fn get_post(_id: u32) -> Result<Post, Never> {
    ///     Ok(Post {
    ///         title: String::new(),
    ///     })
    /// }
    ///
    /// #[actix_web::main]
    /// async fn main() -> std::io::Result<()> {
    ///     let resolver = Data::new(Resolver::new().register::<Post, _, _>(|id| get_post(id)));
    ///
    ///     HttpServer::new(move || {
    ///         App::new()
    ///             .app_data(resolver.clone())
    ///             .route("/api/link", post().to(linked_state_handler))
    ///     })
    ///     .bind(("0.0.0.0", 8080))?
    ///     .run()
    ///     .await
    /// }
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
