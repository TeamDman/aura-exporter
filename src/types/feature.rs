use std::rc::Rc;
use vscodehelper_macros::StringHolder;

#[derive(StringHolder)]
pub struct Feature {
    pub inner: Rc<str>,
}
