use crate::models::municipality::Municipality;
use crate::models::road_history::History;
use crate::models::road_section::RoadSection;
use crate::models::spell_check_status::SpellCheckStatus;
use crate::models::zip_code::ZipCode;
use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;

/// Models a road object from the danish authority for addresses
/// i.e. a 'navngiven vej'
/// The danish authority for addresses, [Danmarks Adresser](https://danmarksadresser.dk/),
/// maintains the national registry for named roads, the
/// [Danmarks Adresseregister (DAR)](https://danmarksadresser.dk/om-adresser/danmarks-adresseregister-dar)
///
/// See the official [definition](https://dawadocs.dataforsyningen.dk/dok/api/navngivenvej#databeskrivelse)
///
/// Fields
///
/// `href`: a URL uniquely referencing the named road
/// `id`: unique road id
/// `status`: named road status in DAR
/// `name`: the official name of the named road, max 40 characters
/// `addressing_name`: an optional, shorter name used then writing addresses on e.g. letters, max 20 characters
/// `history`: defining dates
/// `kommune`: the kommune administrating the named road
/// `spelling`: enum with values: "Udløbet", "Afvist", "Godkendt", "Ikke Kontrolleret"
/// `pronunciation`: the name as pronounced
/// `road_segments`: the pieces of which the named road consists
/// `administrator`: the municipality administrating the named road
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct NamedRoad {
    #[serde(rename = "href")]
    href: Url,

    #[serde(rename = "id")]
    uuid: Uuid,

    #[serde(rename = "darstatus")]
    status: String,

    #[serde(rename = "navn")]
    name: String,

    #[serde(rename = "adresseringsnavn")]
    addressing_name: Option<String>,

    #[serde(rename = "historik")]
    history: History,

    #[serde(rename = "administrerendekommune")]
    administrator: Municipality,

    #[serde(rename = "retskrivningskontrol")]
    spelling: Option<SpellCheckStatus>,

    #[serde(rename = "udtaltvejnavn")]
    pronunciation: Option<String>,

    #[serde(rename = "vejstykker")]
    road_segments: Vec<RoadSection>,

    #[serde(rename = "postnumre")]
    zip_codes: Vec<ZipCode>,
}

impl NamedRoad {
    pub fn href(&self) -> &Url {
        &self.href
    }

    pub fn uuid(&self) -> Uuid {
        self.uuid
    }

    pub fn status(&self) -> &str {
        &self.status
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn addressing_name(&self) -> &Option<String> {
        &self.addressing_name
    }

    pub fn history(&self) -> &History {
        &self.history
    }

    pub fn administrator(&self) -> &Municipality {
        &self.administrator
    }

    pub fn spelling(&self) -> &Option<SpellCheckStatus> {
        &self.spelling
    }

    pub fn pronunciation(&self) -> &Option<String> {
        &self.pronunciation
    }

    pub fn road_segments(&self) -> &Vec<RoadSection> {
        &self.road_segments
    }

    pub fn zip_codes(&self) -> &Vec<ZipCode> {
        &self.zip_codes
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;
    use std::fs;

    fn load_amaliegade_json() -> String {
        fs::read_to_string("tests/data/named_road/amaliegade.json").expect("Error reading file")
    }

    fn load_cph_json() -> String {
        fs::read_to_string("tests/data/named_road/cph.json").expect("Error reading file")
    }

    #[test]
    fn generic_parse_valid() {
        let named_road_json = load_amaliegade_json();

        let named_road: Value =
            serde_json::from_str(named_road_json.as_str()).expect("Error parsing json");

        assert_eq!("Amaliegade", named_road["navn"].as_str().unwrap());
    }

    #[test]
    fn parse_named_roads_array() {
        let json = load_cph_json();
        let named_roads: Vec<NamedRoad> =
            serde_json::from_str(json.as_str()).expect("Error parsing json");
        assert!(named_roads.len() > 0);
    }

    #[test]
    fn parse_valid() {
        let named_road_json = load_amaliegade_json();

        let named_road: NamedRoad = serde_json::from_str(named_road_json.as_str()).expect("Error parsing json");

        assert_eq!("https://api.dataforsyningen.dk/navngivneveje/973179fb-8045-4c6a-a2cd-ab03b31269cf", named_road.href.as_str());
        assert_eq!("973179fb-8045-4c6a-a2cd-ab03b31269cf", named_road.uuid.to_string());
        assert_eq!("gældende", named_road.status);
        assert_eq!("Amaliegade", named_road.name);
        assert_eq!(Some("Amaliegade".to_string()), named_road.addressing_name);

        assert_eq!(Some(SpellCheckStatus::Approved), named_road.spelling);

        assert_eq!(Some(String::from("Amaliegade")), named_road.pronunciation);
    }

    #[test]
    fn parse_administrator() {
        let named_road_json = load_amaliegade_json();

        let named_road: NamedRoad = serde_json::from_str(named_road_json.as_str()).expect("Error parsing json");

        let expected = Municipality::new(
            Url::parse("https://api.dataforsyningen.dk/kommuner/0101").unwrap(),
            "0101",
            Some("København")
        );

        let actual = named_road.administrator;

        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_road_history() {
        let named_road_json = load_amaliegade_json();

        let named_road: NamedRoad = serde_json::from_str(named_road_json.as_str()).expect("Error parsing json");

        let expected = History::new(
            "1900-01-01T12:00:00.000Z",
            "1900-01-01T12:00:00.000Z",
            Some("1900-01-01T12:00:00.000Z"),
            None
        );

        let actual = named_road.history;

        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_road_segment() {
        let named_road_json = load_amaliegade_json();

        let named_road: NamedRoad = serde_json::from_str(named_road_json.as_str()).expect("Error parsing json");

        let first_segment = named_road.road_segments.first().expect("No road segments found");
        assert_eq!("https://api.dataforsyningen.dk/vejstykker/0101/0128", first_segment.href().as_str());
        assert_eq!("0101", first_segment.municipality_code());
        assert_eq!("0128", first_segment.code());
        assert_eq!("5c9636a2-4eae-11e8-93fd-066cff24d637", first_segment.uuid().to_string());
        assert_eq!(3, first_segment.status());
    }

    #[test]
    fn parse_zip_code() {
        let named_road_json = load_amaliegade_json();

        let named_road: NamedRoad = serde_json::from_str(named_road_json.as_str()).expect("Error parsing json");

        let zip_code = ZipCode::new(
            &Url::parse("https://api.dataforsyningen.dk/postnumre/1256").unwrap(),
            "1256",
            Some("København K")
        );
        assert_eq!(&zip_code, named_road.zip_codes.first().unwrap());
    }
}
