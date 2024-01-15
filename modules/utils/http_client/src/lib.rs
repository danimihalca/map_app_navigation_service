pub mod http_utils;

pub trait HttpClient<'a> {
    fn send_request(
        &mut self,
        request: &http_utils::HttpRequest,
        callback: misc::CallbackWrapper<'a, http_utils::HttpResponse>,
    );

    fn call(&self);
}

#[derive(Default)]
pub struct HttpClientImpl<'a> {
    pub callback: misc::CallbackWrapper<'a, http_utils::HttpResponse>,
}

impl<'a> HttpClient<'a> for HttpClientImpl<'a> {
    fn send_request(
        &mut self,
        request: &http_utils::HttpRequest,
        callback: misc::CallbackWrapper<'a, http_utils::HttpResponse>,
    ) {
        self.callback = callback;
    }

    fn call(&self) {
        let resp = http_utils::HttpResponse {
            body: "aaa".to_string(),
            status: http_utils::HttpResponseStatus::Ok200,
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
