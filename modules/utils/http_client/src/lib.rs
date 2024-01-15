pub mod http_utils;

pub trait HttpClient {
    fn send_request(
        &mut self,
        request: &http_utils::HttpRequest,
        callback: misc::CallbackWrapper<http_utils::HttpResponse>,
    );
}

pub struct HttpClientImpl {}

impl HttpClient for HttpClientImpl {
    fn send_request(
        &mut self,
        request: &http_utils::HttpRequest,
        callback: misc::CallbackWrapper<http_utils::HttpResponse>,
    ) {
        let request_builder: reqwest::blocking::RequestBuilder;
        {
            let client = reqwest::blocking::Client::new();
            match request.request_type {
                http_utils::HttpRequestType::GET => {
                    request_builder = client.get(&request.path);
                }
                http_utils::HttpRequestType::POST => {
                    request_builder = client.post(&request.path);
                }
                http_utils::HttpRequestType::PUT => {
                    request_builder = client.put(&request.path);
                }
                http_utils::HttpRequestType::UPDATE => {
                    request_builder = client.patch(&request.path);
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
