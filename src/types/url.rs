use std::rc::Rc;
use reqwest::IntoUrl;
use vscodehelper_macros::StringHolder;

#[derive(StringHolder)]
pub struct Url {
    pub inner: Rc<str>,
}
