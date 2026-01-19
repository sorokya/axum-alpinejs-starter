use axum::{extract::OptionalFromRequestParts, http::request::Parts};

#[derive(Debug, Clone)]
pub struct AlpineRequest {
    pub targets: Vec<String>,
}

impl<S> OptionalFromRequestParts<S> for AlpineRequest
where
    S: Send + Sync,
{
    type Rejection = ();

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> Result<Option<Self>, Self::Rejection> {
        // Check if this is an Alpine request
        let is_alpine = parts
            .headers
            .get("X-Alpine-Request")
            .and_then(|v| v.to_str().ok())
            .map(|v| v.eq_ignore_ascii_case("true"))
            .unwrap_or(false);

        if !is_alpine {
            return Ok(None);
        }

        // Parse targets (space-separated)
        let targets = parts
            .headers
            .get("X-Alpine-Target")
            .and_then(|v| v.to_str().ok())
            .map(|v| {
                v.split_whitespace()
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();

        Ok(Some(AlpineRequest { targets }))
    }
}
