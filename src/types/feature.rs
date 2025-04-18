use std::sync::Arc;
use holda::StringHolda;

#[derive(StringHolda)]
pub struct Feature {
    pub inner: Arc<str>,
}
