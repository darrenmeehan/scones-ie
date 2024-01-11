pub mod github;

use scones::run;

#[tokio::main]
async fn main() {
    run().await
}
