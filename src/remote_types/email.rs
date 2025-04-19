use std::sync::Arc;
use holda::StringHolda;

#[derive(StringHolda)]
pub struct Email {
    pub inner: Arc<str>,
}
