use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, Eq)]
struct PlaceData {
    countries: HashMap<String, String>,
    us_states: HashMap<String, String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PlaceType {
    Country,
    USAState,
    Other,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq, Eq)]
pub struct PlaceDetector {
    country_data: HashMap<String, String>,
    state_data: HashMap<String, String>,
}

lazy_static::lazy_static! {
    static ref LOGGER: () = {
        // Initialize the logger only once
        env_logger::init();
    };
}

impl PlaceDetector {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let _ = *LOGGER;
        let place_data = PlaceData::load_from_json_files().map_err(|err| {
            log::error!(
                "Error while loading countries and USA states data: {:?}",
                err
            );
            err
        })?;
        let country_data = place_data.countries;
        let state_data = place_data.us_states;
        log::info!("Place detector is ready with countries and USA states data.");

        Ok(PlaceDetector {
            country_data,
            state_data,
        })
    }

    pub fn get_place_type(&self, place_name: &str) -> PlaceType {
        if self.country_data.contains_key(place_name) {
            PlaceType::Country
        } else if self.state_data.contains_key(place_name) {
            PlaceType::USAState
        } else {
            PlaceType::Other
        }
    }

    pub fn get_capital(&self, place_name: &str) -> Option<&str> {
        if let Some(capital) = self.country_data.get(place_name) {
            Some(capital)
        } else if let Some(capital) = self.state_data.get(place_name) {
            Some(capital)
        } else {
            None
        }
    }
}

impl PlaceData {
    fn load_from_json_files() -> Result<Self, Box<dyn std::error::Error>> {
        let countries_json = std::fs::read_to_string("src/capitals/countries.json")?;
        let us_states_json = std::fs::read_to_string("src/capitals/us_states.json")?;

        let countries: HashMap<String, String> = serde_json::from_str(&countries_json)?;
        let us_states: HashMap<String, String> = serde_json::from_str(&us_states_json)?;

        Ok(PlaceData {
            countries,
            us_states,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{PlaceDetector, PlaceType};

    #[test]
    fn test_get_place_type_country() {
        let place_detector = PlaceDetector::new().unwrap_or_default();
        // Test if a known country is correctly identified as a country.
        assert_eq!(place_detector.get_place_type("india"), PlaceType::Country);
    }

    #[test]
    fn test_get_place_type_us_state() {
        let place_detector = PlaceDetector::new().unwrap_or_default();
        // Test if a known US state is correctly identified as a US state.
        assert_eq!(place_detector.get_place_type("texas"), PlaceType::USAState);
    }

    #[test]
    fn test_get_place_type_other() {
        let place_detector = PlaceDetector::new().unwrap_or_default();
        // Test if an unknown place is correctly identified as "Other".
        assert_eq!(place_detector.get_place_type("las vegas"), PlaceType::Other);
    }

    #[test]
    fn test_get_capital_country() {
        let place_detector = PlaceDetector::new().unwrap_or_default();
        // Test if the capital of a known country is returned correctly.
        assert_eq!(place_detector.get_capital("switzerland"), Some("bern"));
    }

    #[test]
    fn test_get_capital_us_state() {
        let place_detector = PlaceDetector::new().unwrap_or_default();
        // Test if the capital of a known US state is returned correctly.
        assert_eq!(place_detector.get_capital("texas"), Some("austin"));
    }

    #[test]
    fn test_get_capital_other() {
        let place_detector = PlaceDetector::new().unwrap_or_default();
        // Test if an unknown place returns None for capital.
        assert_eq!(place_detector.get_capital("mumbai"), None);
    }
}
