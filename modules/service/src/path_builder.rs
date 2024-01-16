use crate::service_utils;

pub trait PathBuilder {
    fn withBasePath(self: &mut Self, basePath: String);
    fn withCoordinates(self: &mut Self, coordinates: Vec<service_utils::Coordinate>);
    fn build(self: &mut Self) -> String;
    fn reset(self: &mut Self);
}

pub struct MapboxDirectionsPathBuilder {
    base_path: String,
    coordinates: Vec<service_utils::Coordinate>,
    access_token: String,
}

impl MapboxDirectionsPathBuilder {
    pub fn new(access_token: String) -> Self {
        Self {
            access_token,
            base_path: Default::default(),
            coordinates: Default::default(),
        }
    }
}

impl PathBuilder for MapboxDirectionsPathBuilder {
    fn withBasePath(self: &mut Self, base_path: String) {
        self.base_path = base_path;
    }

    fn withCoordinates(self: &mut Self, coordinates: Vec<service_utils::Coordinate>) {
        self.coordinates = coordinates;
    }

    fn build(self: &mut Self) -> String {
        format!(
            "{}/{}?access_token={}",
            self.base_path,
            self.coordinates
                .iter()
                .map(|c| { format!("{},{}", c.longitude, c.latitude) })
                .collect::<Vec<String>>()
                .join(";"),
            self.access_token
        )
    }

    fn reset(self: &mut Self) {
        self.coordinates.clear();
        self.base_path.clear();
    }
}
