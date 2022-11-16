use tokio_postgres::{Client, Error, NoTls};

pub async fn connect(url: &str) -> Result<Client, Error> {
    let (client, connection) = tokio_postgres::connect(url, NoTls).await.unwrap();

    let err = connection.await;

    match err {
        Ok(_) => return Ok(client),
        Err(e) => return Err(e),
    }
}
