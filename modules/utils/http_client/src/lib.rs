pub mod http_utils;

pub trait HttpClient {
    fn send_request(
        &mut self,
        request: &http_utils::HttpRequest,
        callback: misc::CallbackWrapper<http_utils::HttpResponse>,
    );
}

pub struct HttpClientImpl {
    client: reqwest::blocking::Client,
}

impl Default for HttpClientImpl {
    fn default() -> Self {
        Self {
            client: reqwest::blocking::Client::new(),
        }
    }
}

impl HttpClient for HttpClientImpl {
    fn send_request(
        &mut self,
        request: &http_utils::HttpRequest,
        callback: misc::CallbackWrapper<http_utils::HttpResponse>,
    ) {
        let request_builder: reqwest::blocking::RequestBuilder;
        {
            match request.request_type {
                http_utils::HttpRequestType::GET => {
                    request_builder = self.client.get(&request.path);
                }
                http_utils::HttpRequestType::POST => {
                    request_builder = self.client.post(&request.path);
                }
                http_utils::HttpRequestType::PUT => {
                    request_builder = self.client.put(&request.path);
                }
                http_utils::HttpRequestType::UPDATE => {
                    request_builder = self.client.patch(&request.path);
                }
            }
        }

        let raw_response = request_builder.send().unwrap();

        let response = http_utils::HttpResponse {
            status: http_utils::HttpResponseStatus::Ok200, //TTODO: proper conversions
            body: raw_response.text().unwrap(),
        };

        (callback.callback)(response);
    }
}
