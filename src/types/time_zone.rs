use std::rc::Rc;
use vscodehelper_macros::StringHolder;

#[derive(StringHolder)]
pub struct TimeZone {
    pub inner: Rc<str>,
}
