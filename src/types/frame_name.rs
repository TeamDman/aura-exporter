use std::rc::Rc;
use vscodehelper_macros::StringHolder;

#[derive(StringHolder)]
pub struct FrameName {
    pub inner: Rc<str>,
}
