use crate::blocking::request::Request;
use url::Url;

#[derive(Debug, Default)]
pub struct ChainedConstructor {
    base_url: String,
    query: Option<String>,
    format: Option<String>,
    municipalities: Vec<String>,
}

impl ChainedConstructor {
    pub fn construct(&self) -> Request {
        let mut url = Url::parse(&self.base_url).expect("Invalid base URL");

        if let Some(query) = &self.query {
            url.query_pairs_mut().append_pair("q", query);
        }

        if let Some(format) = &self.format {
            url.query_pairs_mut().append_pair("format", format);
        }

        if !self.municipalities.is_empty() {
            let cities = self.municipalities.join("|");
            let query_prev = url.query().unwrap_or_default();
            let query_new = format!("{}&kommunekode={}", query_prev, cities);
            url.set_query(Some(&query_new));
        }

        Request::new(url)
    }

    pub fn new() -> Self {
        const BASE_URL: &str = "https://api.dataforsyningen.dk/navngivneveje";

        Self {
            base_url: BASE_URL.to_string(),
            query: None,
            format: None,
            municipalities: vec![],
        }
    }

    pub fn query(mut self, query: &str) -> Self {
        self.query = Some(query.to_string());
        self
    }

    pub fn format(mut self, format: &str) -> Self {
        self.format = Some(format.to_string());
        self
    }

    pub fn municipalities(mut self, municipalities: &[&str]) -> Self {
        self.municipalities = municipalities.iter().map(|city| city.to_string()).collect();

        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct() {
        let expected = Url::parse("https://api.dataforsyningen.dk/navngivneveje?q=Amaliegade&format=json&kommunekode=101|185").expect("Invalid URL");

        let constructor = ChainedConstructor::new()
            .query("Amaliegade")
            .format("json")
            .municipalities(&["101", "185"]);

        let request = constructor.construct();

        let actual = request.url().to_owned();

        assert_eq!(expected, actual);
    }
}
