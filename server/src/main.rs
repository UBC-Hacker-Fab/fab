mod interface;
mod devices;
mod state;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tokio::spawn(devices::devices());
    interface::interface().await
}
