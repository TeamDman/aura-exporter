use std::sync::Arc;
use holda::StringHolda;

#[derive(StringHolda)]
pub struct TimeZone {
    pub inner: Arc<str>,
}
