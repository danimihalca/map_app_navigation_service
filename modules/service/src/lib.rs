use http_client::http_utils;

pub mod path_builder;
pub mod service_utils;

pub trait NavigationService {
    fn directions(
        self: &mut Self,
        coordinates: Vec<service_utils::Coordinate>,
        callback: misc::CallbackWrapper<String>,
    );
}

pub struct NavigationServiceImpl {
    http_client: Box<dyn http_client::HttpClient + Send + Sync>,
    path_builder: Box<dyn path_builder::PathBuilder + Send + Sync>,
}

impl NavigationServiceImpl {
    pub fn new(
        http_client: Box<dyn http_client::HttpClient + Send + Sync>,
        path_builder: Box<dyn path_builder::PathBuilder + Send + Sync>,
    ) -> Self {
        Self {
            http_client,
            path_builder,
        }
    }
}

impl NavigationService for NavigationServiceImpl {
    fn directions(
        self: &mut Self,
        coordinates: Vec<service_utils::Coordinate>,
        mut callback: misc::CallbackWrapper<String>,
    ) {
        self.path_builder.reset();

        self.path_builder
            .with_base_path("https://api.mapbox.com/directions/v5/mapbox/driving".to_string());
        self.path_builder.with_coordinates(coordinates);

        let request = http_utils::HttpRequest {
            path: self.path_builder.build(),
            request_type: http_utils::HttpRequestType::GET,
        };

        let http_client_callback = move |response: http_client::http_utils::HttpResponse| {
            (callback.callback)(response.body);
        };

        self.http_client
            .send_request(&request, misc::CallbackWrapper::new(http_client_callback));
    }
}
