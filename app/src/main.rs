use http_client::HttpClient;

fn f(c: &mut dyn http_client::HttpClient) {
    let request = http_client::HttpRequest {
        request_type: http_client::HttpRequestType::GET,
        path: "www.a.com".to_string(),
    };

    let callback = |_response: http_client::HttpResponse| {
        println!("Callback");
    };

    let cbw = misc::CallbackWrapper::new(callback);

    c.sendRequest(&request, cbw);
}

fn main() {
    let x: i32 = 6;
    let mut c = http_client::HttpClientImpl::default();

    f(&mut c);

    c.call();
}
