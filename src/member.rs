use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AllMemberResponse {
    pub members: Vec<Member>,
    pub pagination: Pagination,
    pub request: Request,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Member {
    pub bioguide_id: String,
    pub depiction: Depiction,
    pub district: Option<i32>,
    pub name: String,
    pub party: String,
    pub served: Service,
    pub state: String,
    pub url: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Depiction {
    pub attribution: Option<String>,
    pub image_url: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Service {
    #[serde(rename = "House")]
    pub house: Option<Vec<Record>>,
    #[serde(rename = "Senate")]
    pub senate: Option<Vec<Record>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Record {
    pub end: Option<i32>,
    pub start: i32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pagination {
    pub count: i32,
    pub next: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub content_type: String,
    pub format: String,
}
