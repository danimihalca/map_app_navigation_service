pub struct CallbackWrapper<'a, ResponseType> {
    pub callback: Box<dyn Fn(ResponseType) + 'a>,
}

impl<'a, ResponseType> CallbackWrapper<'a, ResponseType> {
    pub fn new(cb: impl Fn(ResponseType) + 'a) -> Self {
        Self {
            callback: Box::new(cb),
        }
    }
}
