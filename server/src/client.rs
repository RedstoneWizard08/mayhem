use std::{fs::{self, File}, net::SocketAddr, process::Stdio, env::consts::ARCH, path::Path, io::{copy, Read, Cursor}};
use flate2::read::GzDecoder;
use tar::Archive;
use tokio::process::Command;

use include_dir::{include_dir, Dir};
use tempdir::TempDir;

use crate::{config::get_config, util::parse_ip, logging::info};

pub static CLIENT_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/../client/build");

pub fn get_node_download_url() -> String {
    let base = "https://nodejs.org/dist/v16.19.1/node-v16.19.1-linux-";
    let mut arch = "";

    if ARCH == "x86" || ARCH == "x86_64" {
        arch = "x64";
    }

    if ARCH == "arm" {
        arch = "armv7l";
    }

    if ARCH == "aarch64" {
        arch = "arm64";
    }

    return format!("{}{}.tar.gz", base, arch);
}

pub fn get_node_dir_name() -> String {
    let mut arch = "";

    if ARCH == "x86" || ARCH == "x86_64" {
        arch = "x64";
    }

    if ARCH == "arm" {
        arch = "armv7l";
    }

    if ARCH == "aarch64" {
        arch = "arm64";
    }

    return format!("node-v16.19.1-linux-{}", arch);
}

pub async fn download_node(dir: &Path) {
    let url = get_node_download_url();

    println!("Download Node: {}", url.clone());

    let resp = reqwest::get(url).await.unwrap();

    let dest_file = dir.join("node.tar");
    let mut dest = File::create(dest_file.clone()).unwrap();

    let cvec = resp.bytes().await.unwrap().to_vec();
    let content = Cursor::new(cvec);
    let mut gz = GzDecoder::new(content);
    let mut tar_content = Vec::new();

    gz.read_to_end(&mut tar_content).unwrap();

    let mut tar_cursor = Cursor::new(tar_content);

    copy(&mut tar_cursor, &mut dest).unwrap();

    let mut tar = Archive::new(File::open(dest_file).unwrap());

    tar.unpack(dir).unwrap();

    fs::rename(dir.join(get_node_dir_name()), dir.join("node"));
}

pub async fn run_client() {
    let tmp = TempDir::new(".mayhem_client_").unwrap();

    download_node(tmp.path()).await;
    CLIENT_DIR.extract(tmp.path());

    fs::write(tmp.path().join("package.json"), r#"{
    "name": "mayhem-client",
    "version": "0.0.1",
    "type": "module"
}"#);

    let config = get_config().await;
    let ip = parse_ip(config.clone().host);
    let port = config.clone().port;
    let listen_port = port - 1;
    let listen_port_s = listen_port.clone().to_string();
    let address = SocketAddr::from((ip, listen_port));

    info(format!("Client listening on {}", address).as_str());

    let mut cmd = Command::new("node/bin/node");

    cmd.args([tmp.path().join("index.js").to_str().unwrap()])
        .env("PORT", listen_port_s)
        .env("HOST", config.clone().host)
        .current_dir(tmp.path())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let out = cmd.output().await.unwrap();
    
    println!("(Client) STDOUT: {}", String::from_utf8(out.stdout).unwrap());
    println!("(Client) STDERR: {}", String::from_utf8(out.stderr).unwrap());

    info("Exiting client process...");
}
