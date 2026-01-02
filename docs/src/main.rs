use rejoice::App;

pub mod components;
pub mod markdown;

rejoice::routes!();

#[tokio::main]
async fn main() {
    let app = App::new(8080, create_router());
    app.run().await;
}
