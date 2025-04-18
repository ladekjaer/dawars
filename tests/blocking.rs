use dawars::blocking::request::chained_constructor::ChainedConstructor;

#[test]
fn execute_request() {
    let street_name = "Amaliegade";
    let request = ChainedConstructor::new()
        .query(street_name)
        .construct();
    let result = request
        .execute()
        .expect("Failed to execute request.");
    assert!(result.len() > 0);
    
    let response_street_name = result
        .first()
        .expect("Failed to get first result")
        .name();
    assert_eq!(street_name, response_street_name);
}