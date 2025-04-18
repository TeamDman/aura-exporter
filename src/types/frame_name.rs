use std::sync::Arc;
use holda::StringHolda;

#[derive(StringHolda)]
pub struct FrameName {
    pub inner: Arc<str>,
}
