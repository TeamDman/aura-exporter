use std::rc::Rc;
use vscodehelper_macros::StringHolder;

#[derive(StringHolder)]
pub struct AuthToken {
    pub inner: Rc<str>,
}
