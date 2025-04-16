use std::rc::Rc;
use vscodehelper_macros::StringHolder;

#[derive(StringHolder)]
pub struct FileName {
    pub inner: Rc<str>,
}
