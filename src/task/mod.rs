pub mod subreddit;

use url::Url;
use serde_json::Value;

static BASE_URI_API: &str = "https://www.reddit.com";
static URI_AUTH: &str = "https://www.reddit.com/api/v1/authorize";

pub trait RedditTask {
    type Cargo;
    type Payload;
    fn get_request_components(&self) -> (Url, HttpMethod, HttpBody, bool);
    fn handle_response(response: String) -> Self::Cargo;
}

pub enum HttpMethod {
    Get,
    Post,
}

pub enum HttpBody {
    UrlEncoded(String),
    Json(Value),
    Empty,
}
