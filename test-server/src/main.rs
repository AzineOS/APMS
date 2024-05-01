use warp::Filter;

#[tokio::main]
async fn main() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("package" / String)
        .map(|name| format!("TODO: Get {}", name));

    warp::serve(hello)
        .run(([127, 0, 0, 1], 8080))
        .await;
}