use std::sync::Arc;
use holda::StringHolda;

#[derive(StringHolda)]
pub struct UserName {
    pub inner: Arc<str>,
}
