# Map App Navigation Service

## Description

Basic navigation service implemented as a reverse proxy on top of Mapbox's `Directions driving` endpoint.

## Technologies
* Typescript
* Express.js

## Build

### Local build

* Build project:
```
TODO
```
* Run service:
```
TODO
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
