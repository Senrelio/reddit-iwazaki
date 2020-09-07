use crate::task::{RedditTask, HttpMethod, HttpBody};

pub struct RedditClient {}

impl RedditClient {
    pub async fn process_task<T>(task: T) -> T::Cargo
        where T: RedditTask {
        let request_components = task.get_request_components();
        let uri = request_components.0;
        let method = request_components.1;
        let mut request = match method {
            HttpMethod::Get => surf::get(uri),
            HttpMethod::Post => surf::post(uri),
        };
        match request_components.2 {
            HttpBody::Empty => (),
            HttpBody::Json(json) => {
                request = request.body_json(&json).unwrap();
            }
            HttpBody::UrlEncoded(data) => {
                request = request.body_form(&data).unwrap();
            }
        };
        request = request.set_header("User-agent", "macos:reddit-iwazaki:v0.01 (by /u/trabe_lazy)");
        let mut raw_response = request.await.unwrap();
        T::handle_response(raw_response.body_string().await.unwrap())
    }
}