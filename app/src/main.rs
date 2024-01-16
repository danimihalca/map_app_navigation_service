#[derive(Clone)]
struct AppState {
    service: std::sync::Arc<std::sync::Mutex<dyn service::NavigationService + Send + Sync>>,
}

async fn directions(
    axum::extract::State(state): axum::extract::State<AppState>,
    axum::extract::Path(coordinates): axum::extract::Path<String>,
) -> String {
    let coordinates_str = coordinates.split(";");
    let mut output_coordinates = Vec::<service::service_utils::Coordinate>::new();
    for c in coordinates_str {
        let params: Vec<&str> = c.split(",").collect();

        output_coordinates.push(service::service_utils::Coordinate {
            longitude: params[0].parse::<f32>().unwrap(),
            latitude: params[1].parse::<f32>().unwrap(),
        })
    }

    let mut result = String::default();

    let callback = |response: String| {
        result = response;
    };

    let cbw: misc::CallbackWrapper<'_, _> = misc::CallbackWrapper::new(callback);

    let mut service = state.service.lock().expect("mutex was poisoned");

    service.directions(output_coordinates, cbw);

    result
}

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

    let state = AppState {
        service: std::sync::Arc::new(std::sync::Mutex::new(service::NavigationServiceImpl::new(
            http_client,
            path_builder,
        ))),
    };

    let app = axum::Router::new()
        .route(
            "/directions/coordinates=:coordinates",
            axum::routing::get(directions),
        )
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
