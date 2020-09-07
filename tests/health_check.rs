use reddit_iwazaki::task::subreddit::AboutSubreddit;
use reddit_iwazaki::client::RedditClient;

#[async_std::test]
async fn health_check() {
    let r = String::from("aww");
    let task = AboutSubreddit(r);
    let about = RedditClient::process_task(task).await;
    let got = about["kind"].as_str().unwrap();
    let expect = "t5";
    assert_eq!(got, expect);
}

