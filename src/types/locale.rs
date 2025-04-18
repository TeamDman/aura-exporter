use std::sync::Arc;
use holda::StringHolda;

#[derive(StringHolda)]
pub struct Locale {
    pub inner: Arc<str>,
}
