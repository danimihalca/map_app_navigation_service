use std::env;

fn main() {
    let token: String;

    if let Some(arg1) = env::args().nth(1) {
        token = arg1;
    } else {
        panic!("Missing token argument");
    }

    let http_client: Box<dyn http_client::HttpClient> =
        Box::<http_client::HttpClientImpl>::default();

    let path_builder: Box<dyn service::path_builder::PathBuilder> = Box::new(
        service::path_builder::MapboxDirectionsPathBuilder::new(token.to_string()),
    );

    let mut navigation_service: Box<dyn service::NavigationService> = Box::new(
        service::NavigationServiceImpl::new(http_client, path_builder),
    );

    let callback = |response: String| {
        println!("{}", response);
    };
    let cbw = misc::CallbackWrapper::new(callback);

    let coordinates = vec![
        service::service_utils::Coordinate {
            longitude: -122.42,
            latitude: 37.78,
        },
        service::service_utils::Coordinate {
            longitude: -77.03,
            latitude: 38.91,
        },
    ];

    navigation_service.directions(coordinates, cbw);
}
