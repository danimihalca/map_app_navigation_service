use http_client::HttpClient;

fn f(c: &mut dyn http_client::HttpClient) {
    let request = http_client::http_utils::HttpRequest {
        request_type: http_client::http_utils::HttpRequestType::GET,
        path: "www.a.com".to_string(),
    };

    let callback = |_response: http_client::http_utils::HttpResponse| {
        println!("Callback");
    };

    let cbw = misc::CallbackWrapper::new(callback);

    c.send_request(&request, cbw);
}

fn main() {
    let mut c = http_client::HttpClientImpl::default();

    f(&mut c);

    c.call();
}
