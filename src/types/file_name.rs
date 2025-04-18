use std::sync::Arc;
use holda::StringHolda;

#[derive(StringHolda)]
pub struct FileName {
    pub inner: Arc<str>,
}
