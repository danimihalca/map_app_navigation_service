pub mod http_utils;

#[mockall::automock]
pub trait HttpClient {
    fn send_request<'a>(
        &mut self,
        request: &http_utils::HttpRequest,
        callback: misc::CallbackWrapper<'a, http_utils::HttpResponse>,
    );
}

pub struct HttpClientImpl {}

impl HttpClient for HttpClientImpl {
    fn send_request(
        &mut self,
        request: &http_utils::HttpRequest,
        mut callback: misc::CallbackWrapper<http_utils::HttpResponse>,
    ) {
        let output_request: ureq::Request;
        {
            match request.request_type {
                http_utils::HttpRequestType::GET => {
                    output_request = ureq::get(&request.path);
                }
                http_utils::HttpRequestType::POST => {
                    output_request = ureq::post(&request.path);
                }
                http_utils::HttpRequestType::PUT => {
                    output_request = ureq::put(&request.path);
                }
                http_utils::HttpRequestType::UPDATE => {
                    output_request = ureq::patch(&request.path);
                }
            }
        }

        let raw_response = output_request.call().unwrap();

        let response = http_utils::HttpResponse {
            status: http_utils::HttpResponseStatus::Ok200, //TTODO: proper conversions
            body: raw_response.into_string().unwrap(),
        };

        (callback.callback)(response);
    }
}
