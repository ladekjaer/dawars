use std::error::Error;
use url::Url;
use crate::blocking::request_builder::RequestBuilder;
use crate::models::named_road::NamedRoad;

pub struct Request {
    url: Url,
}

impl Request {
    pub fn execute(&self) -> Result<Vec<NamedRoad>, Box<dyn Error>> {
        let response = reqwest::blocking::get(self.url().as_str())?
            .json::<Vec<NamedRoad>>()?;
        Ok(response)
    }

    pub fn builder() -> RequestBuilder {
        RequestBuilder::default()
    }

    pub fn new(url: Url) -> Self {
        Self { url }
    }

    pub fn url(&self) -> &Url {
        &self.url
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder() {
        let request = RequestBuilder::new()
            .build();
        assert_eq!(request.url().scheme(), "https");
    }

    #[test]
    fn test_new() {
        let url = Url::parse("https://api.dataforsyningen.dk/navngivneveje").expect("Invalid URL");
        let expected = "https://api.dataforsyningen.dk/navngivneveje";
        let actual = Request::new(url).url().to_string();
        assert_eq!(expected, &actual);
    }

    #[test]
    fn test_url() {
        let request = RequestBuilder::new()
            .build();
        let expected = "https://api.dataforsyningen.dk/navngivneveje";
        let actual = request.url().to_string();
        assert_eq!(expected, &actual);
    }
}
