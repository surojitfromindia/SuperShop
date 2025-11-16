use axum::extract::{FromRef, FromRequestParts};
use axum::http::HeaderMap;
use axum::http::request::Parts;
use super_shop_backend::{AccessContext, AppState, AuthContextInput};

pub struct AccessContextExtractor(pub AccessContext);


impl<S> FromRequestParts<S> for AccessContextExtractor
where
    S: Send + Sync + 'static,
    AppState: FromRef<S>,
{
    type Rejection = String;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let headers = &parts.headers;

        let token = extract_token(headers); // custom function

        let app_state = AppState::from_ref(state);
        let access_ctx = AccessContext::init(
            AuthContextInput {
                token,
                app_state: app_state.clone(),
            },
            app_state,
        ).await.map_err(|e| e.to_string())?;

        Ok(AccessContextExtractor(access_ctx))
    }
}


fn extract_token(headers: &HeaderMap) -> String {
    // todo: later fix the error handling.
    let header = headers.get(axum::http::header::AUTHORIZATION).expect("No authorization header");
    let value = header.to_str().expect("No authorization header");
    let token = value.strip_prefix("Bearer ").expect("Missing token");
    token.to_string()
}
