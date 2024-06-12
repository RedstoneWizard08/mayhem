use anyhow::Result;

#[tokio::main]
pub async fn main() -> Result<()> {
    mayhem_server::start()
}
