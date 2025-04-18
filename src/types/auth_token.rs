use std::sync::Arc;
use holda::StringHolda;

#[derive(StringHolda)]
pub struct AuthToken {
    pub inner: Arc<str>,
}
