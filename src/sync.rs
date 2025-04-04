use crate::models::named_road::NamedRoad;

pub fn query(name: &str) -> Result<Vec<NamedRoad>, Box<dyn std::error::Error>> {
    let url = build_url(name);
    let response = reqwest::blocking::get(url)?.json::<Vec<NamedRoad>>()?;

    Ok(response)
}

fn build_url(name: &str) -> String {
    const BASE_URL: &str =
        "https://api.dataforsyningen.dk/navngivneveje?kommunekode=101|185&format=json&valider";
    format!("{}&q={}", BASE_URL, name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_is_ok() {
        let name = "Amaliegade";
        let result = query(name);
        assert!(result.is_ok());
    }

    #[test]
    fn test_query_non_empty() {
        let name = "Amaliegade";
        let result = query(name).unwrap();
        assert!(!result.is_empty());
    }

    #[test]
    fn test_query_len_is_1() {
        let name = "Amaliegade";
        let result = query(name).unwrap();
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_query_correct_name() {
        let name = "Amaliegade";
        let result = query(name).unwrap();
        let first = result.first().unwrap();
        assert_eq!(first.name(), name);
    }
}
