use serde::Deserialize;

// override envs (jwt duration)
// optional global options for headers
// optional headers
// keep cookings if response has any and send them on requests
// can use oneshot for every step

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct TestSection {
    pub name: Option<String>,
    pub description: Option<String>,
    pub req_type: Option<String>,
    pub req_body: Option<String>,
    pub resp_body: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct Overrides {

}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct TestConfig {
    #[serde(rename = "section")]
    pub queries: Vec<TestSection>,
    pub overrides: Overrides,
}