use kiah_stack::{
    app::App,
    db::{PoolConfig, create_pool},
    env::dotenv,
};
use std::time::Duration;

#[tokio::main]
async fn main() {
    let pool = create_pool(PoolConfig {
        db_url: dotenv!("DATABASE_URL").to_string(),
        max_connections: 10,
        acquire_timeout: Duration::from_secs(5),
        idle_timeout: Duration::from_secs(300),
        max_lifetime: Duration::from_secs(1800),
    })
    .await;

    let app = App::new(8080, pool);
    app.run().await;
}
