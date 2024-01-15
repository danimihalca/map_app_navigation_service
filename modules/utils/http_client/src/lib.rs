
pub enum HttpRequestType {
    GET,
    POST,
    PUT,
    UPDATE,
}

pub struct HttpRequest {
    pub request_type: HttpRequestType,
    pub path: String,
}

pub enum HttpResponseStatus {
    Ok200,
    NotFound404,
}

pub struct HttpResponse {
    pub status: HttpResponseStatus,
    pub body: String,
}

pub trait HttpClient<'a> {
    fn sendRequest(
        &mut self,
        request: &HttpRequest,
        callback: misc::CallbackWrapper<'a, HttpResponse>,
    );

    fn call(&self);
}

#[derive(Default)]
pub struct HttpClientImpl<'a> {
    pub callback: misc::CallbackWrapper<'a, HttpResponse>,
}

impl<'a> HttpClient<'a> for HttpClientImpl<'a> {
    fn sendRequest(
        &mut self,
        request: &HttpRequest,
        callback: misc::CallbackWrapper<'a, HttpResponse>,
    ) {
        self.callback = callback;
    }

    fn call(&self) {
        let resp = HttpResponse {
            body: "aaa".to_string(),
            status: HttpResponseStatus::Ok200,
        };

        match &self.callback.callback {
            Some(callbackBox) => {
                callbackBox(resp);
            }
            None => {
                println!("No callback set");
            }
        }
    }
}
