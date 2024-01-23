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
        self.path_builder
            .with_parameter("steps".to_string(), "true".to_string());
        self.path_builder
            .with_parameter("geometries".to_string(), "geojson".to_string());

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

#[cfg(test)]
mod test {

    use misc::CallbackWrapper;

    use super::*;

    #[test]
    fn basic_call() {
        let mut mock_path_builder = Box::<path_builder::MockPathBuilder>::default();
        let mut mock_http_client = Box::<http_client::MockHttpClient>::default();

        mock_path_builder.expect_reset().return_const(());
        mock_path_builder
            .expect_with_base_path()
            .with(mockall::predicate::eq(
                "https://api.mapbox.com/directions/v5/mapbox/driving".to_string(),
            ))
            .return_const(());
        mock_path_builder
            .expect_with_coordinates()
            .with(mockall::predicate::eq(vec![
                service_utils::Coordinate {
                    longitude: 5.5,
                    latitude: 6.6,
                },
                service_utils::Coordinate {
                    longitude: 7.7,
                    latitude: 8.8,
                },
            ]))
            .return_const(());

        mock_path_builder
            .expect_with_parameter()
            .withf(|key, value| key == "geometries" && value == "geojson")
            .return_const(());

        mock_path_builder
            .expect_with_parameter()
            .withf(|key, value| key == "steps" && value == "true")
            .return_const(());

        mock_path_builder
            .expect_build()
            .once()
            .return_const("<DUMMY_PATH>");

        mock_http_client
            .expect_send_request()
            .once()
            .withf_st(|request, _| {
                request.path == "<DUMMY_PATH>"
                    && request.request_type == http_utils::HttpRequestType::GET
            })
            .returning_st(
                |_request: &http_utils::HttpRequest,
                 mut callback: CallbackWrapper<http_utils::HttpResponse>| {
                    let response = http_utils::HttpResponse {
                        status: http_utils::HttpResponseStatus::Ok200,
                        body: "<BODY>".to_string(),
                    };
                    //Trigger received internal callback from service here, simulate a response
                    (callback.callback)(response);
                    ()
                },
            );

        let mut service: NavigationServiceImpl =
            NavigationServiceImpl::new(mock_http_client, mock_path_builder);

        let mut callback_triggered = false;
        let mut callback_response = String::default();
        let callback = CallbackWrapper::new(|response| {
            callback_triggered = true;
            callback_response = response;
        });

        service.directions(
            vec![
                service_utils::Coordinate {
                    longitude: 5.5,
                    latitude: 6.6,
                },
                service_utils::Coordinate {
                    longitude: 7.7,
                    latitude: 8.8,
                },
            ],
            callback,
        );

        assert!(callback_triggered);
        assert_eq!(callback_response, "<BODY>")
    }
}
