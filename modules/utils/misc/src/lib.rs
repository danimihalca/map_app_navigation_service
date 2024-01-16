pub struct CallbackWrapper<'a, ResponseType> {
    pub callback: Box<dyn FnMut(ResponseType) + 'a>,
}

impl<'a, ResponseType> CallbackWrapper<'a, ResponseType> {
    pub fn new(cb: impl FnMut(ResponseType) + 'a) -> Self {
        Self {
            callback: Box::new(cb),
        }
    }
}
