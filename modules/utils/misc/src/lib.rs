
pub struct CallbackWrapper<'a, ResponseType> {
    pub callback: Option<Box<dyn Fn(ResponseType) + 'a>>,
}

impl<'a, ResponseType> Default for CallbackWrapper<'a, ResponseType> {
    fn default() -> Self {
        Self { callback: None }
    }
}

impl<'a, ResponseType> CallbackWrapper<'a, ResponseType> {
    pub fn new(cb: impl Fn(ResponseType) + 'a) -> Self {
        Self {
            callback: Some(Box::new(cb)),
        }
    }
}
