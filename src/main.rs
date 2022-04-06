use warp::{ws::WebSocket, Filter, Rejection, Reply};

#[tokio::main]
async fn main() {
    let metrics_route = warp::path!("metrics").and_then(metrics_handler);

    println!("Started on port 8080");
    warp::serve(metrics_route)
        .run(([0, 0, 0, 0], 8081))
        .await;
}

async fn metrics_handler() -> Result<impl Reply, Rejection> {
    Ok("hello!")
}
