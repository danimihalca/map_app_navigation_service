#[derive(Clone)]
pub struct AppState {
    pub service: std::sync::Arc<std::sync::Mutex<dyn service::NavigationService + Send + Sync>>,
}

pub async fn run(
    service: std::sync::Arc<std::sync::Mutex<dyn service::NavigationService + Send + Sync>>,
) {
    let state = AppState { service };

    let cors = tower_http::cors::CorsLayer::new()
        .allow_methods([http::Method::GET])
        .allow_origin(tower_http::cors::Any)
        .allow_headers([http::header::CONTENT_TYPE]);

    let app: axum::Router = axum::Router::new()
        .route(
            "/directions/coordinates=:coordinates",
            axum::routing::get(directions),
        )
        .layer(cors)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn parse_coordinates(coordinates: String) -> Vec<service::service_utils::Coordinate> {
    let coordinates_str_vec = coordinates.split(";");
    let mut output_coordinates = Vec::<service::service_utils::Coordinate>::new();
    for coordinate_pair in coordinates_str_vec {
        let params: Vec<&str> = coordinate_pair.split(",").collect();

        output_coordinates.push(service::service_utils::Coordinate {
            longitude: params[0].parse::<f32>().unwrap(),
            latitude: params[1].parse::<f32>().unwrap(),
        })
    }

    output_coordinates
}

pub async fn directions(
    axum::extract::State(state): axum::extract::State<AppState>,
    axum::extract::Path(coordinates): axum::extract::Path<String>,
) -> String {
    let output_coordinates = parse_coordinates(coordinates);

    let mut result = String::default();

    let callback: misc::CallbackWrapper<'_, _> = misc::CallbackWrapper::new(|response: String| {
        result = response;
    });

    let mut service = state.service.lock().unwrap();

    service.directions(output_coordinates, callback);

    result
}
