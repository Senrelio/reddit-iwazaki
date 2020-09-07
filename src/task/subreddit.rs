use crate::task::{RedditTask, BASE_URI_API, HttpBody, HttpMethod};
use serde_json::Value;
use url::Url;
use std::str::FromStr;
use crate::entity::subreddit::Subreddit;

pub struct AboutSubreddit(pub String);

impl RedditTask for AboutSubreddit {
    type Cargo = Value;
    type Payload = ();

    fn get_request_components(&self) -> (Url, HttpMethod, HttpBody, bool) {
        let uri = Url::from_str(format!("{}/r/{}/about.json", BASE_URI_API, self.0).as_str()).unwrap();
        let http_method = HttpMethod::Get;
        let http_body = HttpBody::Empty;
        let require_auth = false;
        (uri, http_method, http_body, require_auth)
    }

    fn handle_response(response: String) -> Self::Cargo {
        println!("{}", response);
        serde_json::from_str(&response).unwrap()
    }
}

pub enum AllWhere {
    Popular,
    New,
    Gold,
    Default,
}

pub struct GetAllSubreddits(AllWhere);

impl RedditTask for GetAllSubreddits {
    type Cargo = Vec<Subreddit>;
    type Payload = ();

    fn get_request_components(&self) -> (Url, HttpMethod, HttpBody, bool) {
        let a_where = match self.0 {
            AllWhere::Default => "default",
            AllWhere::New => "new",
            AllWhere::Gold => "gold",
            AllWhere::Popular => "popular",
        };
        let url = Url::from_str(format!("{}/subreddits/{}", BASE_URI_API, a_where).as_str());
        (url.unwrap(), HttpMethod::Get, HttpBody::Empty, false)
    }

    fn handle_response(response: String) -> Self::Cargo {
        println!("{}", response);
        vec![]
    }
}

