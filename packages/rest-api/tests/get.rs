#[derive(serde::Serialize)]
struct QueryParamsTest {
    key: String,
    value: String,
}

fn create_get_request_with_params<P>(url: &str, query_params: &P) -> reqwest::Request
where
    P: serde::Serialize + ?Sized,
{
    let client = reqwest::Client::builder().build().unwrap();

    let req = client.get(url).query(query_params);

    req.build().unwrap()
}

#[test]
fn test_query_params_encoding() {
    let url = "https://api.example.com/test";
    let query_params = QueryParamsTest {
        key: "key with spaces".to_string(),
        value: "value&special=chars".to_string(),
    };
    let expected = format!("{url}?key=key+with+spaces&value=value%26special%3Dchars");

    let res = create_get_request_with_params(url, &query_params);
    assert_eq!(res.url().to_string(), expected);
}

#[test]
fn test_query_params_with_unicode() {
    let url = "https://api.example.com/test";
    let query_params = QueryParamsTest {
        key: "한글키".to_string(),
        value: "한글값".to_string(),
    };
    let expected =
        format!("{url}?key=%ED%95%9C%EA%B8%80%ED%82%A4&value=%ED%95%9C%EA%B8%80%EA%B0%92");

    let res = create_get_request_with_params(url, &query_params);
    assert_eq!(res.url().to_string(), expected);
}

#[test]
fn test_query_params_with_special_chars() {
    let url = "https://api.example.com/test";
    let query_params = QueryParamsTest {
        key: "key!@#$%".to_string(),
        value: "value^&*()".to_string(),
    };
    let expected = format!("{url}?key=key%21%40%23%24%25&value=value%5E%26*%28%29");
    let res = create_get_request_with_params(url, &query_params);
    assert_eq!(res.url().to_string(), expected);
}

#[test]
fn test_query_params_empty() {
    let url = "https://api.example.com/test";
    let res = create_get_request_with_params(url, &None::<()>);
    assert_eq!(res.url().to_string(), url);
}
