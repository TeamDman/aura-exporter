use std::rc::Rc;
use vscodehelper_macros::StringHolder;

#[derive(StringHolder)]
pub struct UserName {
    pub inner: Rc<str>,
}
