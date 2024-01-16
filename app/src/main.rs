#[tokio::main]
async fn main() {
    let token: String;

    if let Some(arg1) = std::env::args().nth(1) {
        token = arg1;
    } else {
        panic!("Missing token argument");
    }

    let http_client = Box::new(http_client::HttpClientImpl {});

    let path_builder = Box::new(service::path_builder::MapboxDirectionsPathBuilder::new(
        token.to_string(),
    ));

    let service = std::sync::Arc::new(std::sync::Mutex::new(service::NavigationServiceImpl::new(
        http_client,
        path_builder,
    )));

    endpoint::run(service).await;
}
