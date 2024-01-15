fn f(c: &mut dyn http_client::HttpClient) {
    let request = http_client::http_utils::HttpRequest {
        request_type: http_client::http_utils::HttpRequestType::GET,
        path: "https://www.google.com".to_string(),
    };

    let callback = |response: http_client::http_utils::HttpResponse| {
        println!("{}", response.body);
    };

    let cbw = misc::CallbackWrapper::new(callback);

    c.send_request(&request, cbw);
}

fn main() {
    let mut c = http_client::HttpClientImpl {};

    f(&mut c);
}
