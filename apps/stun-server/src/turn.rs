use std::{collections::HashMap, sync::Arc, net::{IpAddr, SocketAddr}, str::FromStr};

use anyhow::Result;
use tokio::{net::UdpSocket, time::Duration, signal};
use webrtc::{turn::{auth::{generate_auth_key, AuthHandler}, server::{Server, config::{ServerConfig, ConnConfig}}, relay::relay_static::RelayAddressGeneratorStatic, Error}, util::vnet::net::Net};

pub struct MyAuthHandler {
    cred_map: HashMap<String, Vec<u8>>,
}

impl MyAuthHandler {
    fn new(cred_map: HashMap<String, Vec<u8>>) -> Self {
        MyAuthHandler { cred_map }
    }
}

impl AuthHandler for MyAuthHandler {
    fn auth_handle(
        &self,
        username: &str,
        _realm: &str,
        _src_addr: SocketAddr,
    ) -> Result<Vec<u8>, Error> {
        if let Some(pw) = self.cred_map.get(username) {
            //log::debug!("username={}, password={:?}", username, pw);

            return Ok(pw.to_vec());
        } else {
            return Err(Error::ErrFakeErr);
        }
    }
}

pub async fn start_turn_server() -> Result<()> {
    let public_ip = "155.248.206.62";
    let port = 4002;
    let users = "test=testing";
    let realm = "mayhem-webrtc-turn";

    let creds: Vec<&str> = users.split(',').collect();
    let mut cred_map = HashMap::new();

    for user in creds {
        let cred: Vec<&str> = user.splitn(2, '=').collect();
        let key = generate_auth_key(cred[0], realm, cred[1]);

        cred_map.insert(cred[0].to_owned(), key);
    }

    let conn = Arc::new(UdpSocket::bind(format!("0.0.0.0:{}", port)).await?);

    println!("Listening {}...", conn.local_addr()?);

    let server = Server::new(ServerConfig {
        conn_configs: vec![
            ConnConfig {
                conn,

                relay_addr_generator: Box::new(RelayAddressGeneratorStatic {
                    relay_address: IpAddr::from_str(public_ip)?,
                    address: "0.0.0.0".to_owned(),
                    net: Arc::new(Net::new(None)),
                }),
            },
        ],

        realm: realm.to_owned(),
        auth_handler: Arc::new(MyAuthHandler::new(cred_map)),
        channel_bind_timeout: Duration::from_secs(0),
    }).await?;

    println!("Waiting for CTRL+C...");

    signal::ctrl_c().await.expect("failed to listen for event");

    println!("\nClosing connection now...");

    server.close().await?;

    return Ok(());
}
