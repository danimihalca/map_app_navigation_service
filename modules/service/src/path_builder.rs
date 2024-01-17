use std::collections::HashMap;

use crate::service_utils;

pub trait PathBuilder {
    fn with_base_path(self: &mut Self, base_path: String);
    fn with_coordinates(self: &mut Self, coordinates: Vec<service_utils::Coordinate>);
    fn with_parameter(self: &mut Self, key: String, value: String);
    fn build(self: &mut Self) -> String;
    fn reset(self: &mut Self);
}

pub struct MapboxDirectionsPathBuilder {
    base_path: String,
    coordinates: Vec<service_utils::Coordinate>,
    access_token: String,
    parameters: HashMap<String, String>,
}

impl MapboxDirectionsPathBuilder {
    pub fn new(access_token: String) -> Self {
        Self {
            access_token,
            base_path: Default::default(),
            coordinates: Default::default(),
            parameters: Default::default(),
        }
    }
}

impl PathBuilder for MapboxDirectionsPathBuilder {
    fn with_base_path(self: &mut Self, base_path: String) {
        self.base_path = base_path;
    }

    fn with_coordinates(self: &mut Self, coordinates: Vec<service_utils::Coordinate>) {
        self.coordinates = coordinates;
    }

    fn with_parameter(self: &mut Self, key: String, value: String) {
        self.parameters.insert(key, value);
    }

    fn build(self: &mut Self) -> String {
        format!(
            "{}/{}?access_token={}&{}",
            self.base_path,
            self.coordinates
                .iter()
                .map(|c| { format!("{},{}", c.longitude, c.latitude) })
                .collect::<Vec<String>>()
                .join(";"),
            self.access_token,
            self.parameters
                .iter()
                .map(|(key, value)| { format!("{}={}", key, value) })
                .collect::<Vec<String>>()
                .join("&")
        )
    }

    fn reset(self: &mut Self) {
        self.coordinates.clear();
        self.base_path.clear();
    }
}
