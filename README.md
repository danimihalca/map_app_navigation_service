# Map App Navigation Service

## Description

Basic navigation service implemented as a reverse proxy on top of Mapbox's `Directions driving` endpoint.

## Technologies
* Rust
* ureq
* Axum
* Mockall

## Architecture

### Class Diagram

![./class_diagram.plantuml](https://www.plantuml.com/plantuml/proxy?cache=no&src=https://raw.githubusercontent.com/danimihalca/map_app_navigation_service/main/class_diagram.plantuml)

## Sequence Diagrams

### Navigation Directions API
![./navigation_directions_sequence_diagram.plantuml](https://www.plantuml.com/plantuml/proxy?cache=no&src=https://raw.githubusercontent.com/danimihalca/map_app_navigation_service/main/navigation_directions_sequence_diagram.plantuml)


## Build

### Local build
* Prerequisites:
    - Rust 1.75.9

* Build project:
```
cargo build
```
* Run service:
```
cargo run -- <MAPBOX_TOKEN>
```
* Optional: Run unit tests (only in Debug):
```
cargo test
```

### Docker build

* Build Docker image:
```
docker build --target runner -t <NAVIGATION_SERVICE_IMG> .
```
* Run Docker container:
```
docker run -p 3000:3000 -e MAPBOX_TOKEN=<MAPBOX_TOKEN> -dit <NAVIGATION_SERVICE_IMG>
```