use crate::service_utils;
use mockall::automock;
use std::collections::BTreeMap;

#[automock]
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
    parameters: BTreeMap<String, String>,
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
        self.parameters.clear();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn normal_build() {
        let mut builder = MapboxDirectionsPathBuilder::new("<TOKEN>".to_string());

        builder.with_base_path("<BASE_PATH>".to_string());
        builder.with_coordinates(vec![
            service_utils::Coordinate {
                longitude: 1.1,
                latitude: 2.2,
            },
            service_utils::Coordinate {
                longitude: 3.3,
                latitude: 4.4,
            },
        ]);

        builder.with_parameter("<PARAM_KEY1>".to_string(), "<PARAM_VAL1>".to_string());

        let result = builder.build();

        assert_eq!(
            result,
            "<BASE_PATH>/1.1,2.2;3.3,4.4?access_token=<TOKEN>&<PARAM_KEY1>=<PARAM_VAL1>"
        );
    }

    #[test]
    fn multiple_build() {
        let mut builder = MapboxDirectionsPathBuilder::new("<TOKEN>".to_string());

        builder.with_base_path("<BASE_PATH1>".to_string());
        builder.with_coordinates(vec![
            service_utils::Coordinate {
                longitude: 1.1,
                latitude: 2.2,
            },
            service_utils::Coordinate {
                longitude: 3.3,
                latitude: 4.4,
            },
        ]);

        builder.with_parameter("<PARAM_KEY1>".to_string(), "<PARAM_VAL1>".to_string());

        let result = builder.build();

        assert_eq!(
            result,
            "<BASE_PATH1>/1.1,2.2;3.3,4.4?access_token=<TOKEN>&<PARAM_KEY1>=<PARAM_VAL1>"
        );

        builder.reset();

        builder.with_base_path("<BASE_PATH2>".to_string());
        builder.with_coordinates(vec![
            service_utils::Coordinate {
                longitude: 5.5,
                latitude: 6.6,
            },
            service_utils::Coordinate {
                longitude: 7.7,
                latitude: 8.8,
            },
        ]);

        builder.with_parameter("<PARAM_KEY2>".to_string(), "<PARAM_VAL2>".to_string());
        builder.with_parameter("<PARAM_KEY3>".to_string(), "<PARAM_VAL3>".to_string());

        let result = builder.build();

        assert_eq!(
            result,
            "<BASE_PATH2>/5.5,6.6;7.7,8.8?access_token=<TOKEN>&<PARAM_KEY2>=<PARAM_VAL2>&<PARAM_KEY3>=<PARAM_VAL3>"
        );
    }
}
