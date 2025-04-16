use std::rc::Rc;
use vscodehelper_macros::StringHolder;

#[derive(StringHolder)]
pub struct Locale {
    pub inner: Rc<str>,
}
