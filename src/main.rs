use futures::future;

use tgbot::start_server;

#[tokio::main]
async fn main() {
    start_server().await;

    // wait created threads - they must not terminate
    future::pending::<()>().await;
}
