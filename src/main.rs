mod tools;
use poem::{
    listener::TcpListener,
    Server,
    Route,
    endpoint::EmbeddedFilesEndpoint,
    get
};
use rust_embed::RustEmbed;

mod routers;

#[derive(RustEmbed)]
#[folder = "images"]
struct Files;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), std::io::Error> {
    let app = Route::new().at("/", get(routers::index::index)).nest("/images", EmbeddedFilesEndpoint::<Files>::new());
    println!("Server running on port 3000");
    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(app)
        .await
}