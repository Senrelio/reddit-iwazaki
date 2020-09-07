use tiny_http::Server;
use regex::Regex;

pub struct User {
    token: String,
}

impl User {
    async fn prepare_for_redirect() -> Option<String> {
        let server = Server::http("localhost:8211").unwrap();
        for request in server.incoming_requests() {
            let url = request.url();
            let re = Regex::new(r"").unwrap();
            if re.is_match(url) {
                return Some(String::from("test"));
            }
        };
        None
    }
}