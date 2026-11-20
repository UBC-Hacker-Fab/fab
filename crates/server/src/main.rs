mod interface;
mod devices;
mod state;

use std::sync::RwLock;

static tools: RwLock<Vec<RwLock<Option<state::Tool>>>> = RwLock::new(Vec::new());

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tokio::spawn(devices::devices(&tools));
    interface::interface(&tools).await
}
