use crate::prelude::*;

pub(crate) fn router() -> Router<AppStateRef> {
    Router::new()
        .route("/", get(|| async { "Hello, World!" }))
}

