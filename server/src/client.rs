use flate2::read::GzDecoder;
use indicatif::{ProgressBar, ProgressStyle};
use std::{
    cmp::min,
    env::consts::ARCH,
    fs::{self, File},
    io::{copy, Cursor, Read, Write},
    net::SocketAddr,
    path::Path,
    process::Stdio,
};
use tar::Archive;
use tokio::process::Command;
use tokio_stream::StreamExt;

use include_dir::{include_dir, Dir};
use tempdir::TempDir;

use crate::{config::get_config, logging::info, util::parse_ip};

pub static CLIENT_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/../build");

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

    info("Downloading node...");

    let resp = reqwest::Client::new()
        .get(url.clone())
        .send()
        .await
        .unwrap();
    let total_size = resp.content_length().unwrap();

    let pb = ProgressBar::new(total_size);

    pb.set_style(ProgressStyle::default_bar()
        .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
        .unwrap()
        .progress_chars("#>-"));

    pb.set_message(format!("Downloading {}", url));

    let dl_dest_file = dir.join("node.tar.gz");

    let mut file = File::create(dl_dest_file.clone()).unwrap();
    let mut downloaded: u64 = 0;
    let mut stream = resp.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item.or(Err("Error while downloading file")).unwrap();

        file.write_all(&chunk)
            .or(Err("Error while writing to file"))
            .unwrap();

        let new = min(downloaded + (chunk.len() as u64), total_size);

        downloaded = new;

        pb.set_position(new);
    }

    pb.finish_with_message(format!(
        "Downloaded {} to {}",
        url,
        dl_dest_file.as_os_str().to_str().unwrap()
    ));

    info("Setting up node install...");

    let dest_file = dir.join("node.tar");
    let mut in_file = File::open(dl_dest_file).unwrap();
    let mut in_bytes = Vec::new();

    in_file.read_to_end(&mut in_bytes);

    let mut dest = File::create(dest_file.clone()).unwrap();
    let content = Cursor::new(in_bytes);

    info("Extracting node (1/2)...");

    let mut gz = GzDecoder::new(content);
    let mut tar_content = Vec::new();

    gz.read_to_end(&mut tar_content).unwrap();

    let mut tar_cursor = Cursor::new(tar_content);

    copy(&mut tar_cursor, &mut dest).unwrap();

    info("Extracting node (2/2)...");

    let mut tar = Archive::new(File::open(dest_file).unwrap());

    tar.unpack(dir).unwrap();

    fs::rename(dir.join(get_node_dir_name()), dir.join("node"));
}

pub async fn run_client() {
    let tmp = TempDir::new("mayhem").unwrap();

    download_node(tmp.path()).await;

    info("Extracting frontend files...");

    CLIENT_DIR.extract(tmp.path());

    fs::write(
        tmp.path().join("package.json"),
        r#"{
    "name": "mayhem-client",
    "version": "0.0.1",
    "type": "module"
}"#,
    );

    let config = get_config().await;
    let ip = parse_ip(config.clone().host);
    let port = config.clone().port;
    let listen_port = port + 1;
    let listen_port_s = listen_port.clone().to_string();
    let address = SocketAddr::from((ip, listen_port));

    let file_stdout = File::create(tmp.path().join("stdout_install.log")).unwrap();
    let file_stderr = File::create(tmp.path().join("stderr_install.log")).unwrap();

    let stdout = Stdio::from(file_stdout);
    let stderr = Stdio::from(file_stderr);

    info("Installing moment.js...");

    let install = Command::new("node/bin/npm")
        .arg("install")
        .arg("moment")
        .current_dir(tmp.path())
        .stdout(stdout)
        .stderr(stderr)
        .output()
        .await
        .unwrap();

    println!(
        "(NPM) STDOUT: {}",
        String::from_utf8(install.stdout).unwrap()
    );

    println!(
        "(NPM) STDERR: {}",
        String::from_utf8(install.stderr).unwrap()
    );

    let file_stdout = File::create(tmp.path().join("stdout.log")).unwrap();
    let file_stderr = File::create(tmp.path().join("stderr.log")).unwrap();

    let stdout = Stdio::from(file_stdout);
    let stderr = Stdio::from(file_stderr);

    info(format!("Client listening on {}", address).as_str());

    let out = Command::new("node/bin/node")
        .arg(tmp.path().join("index.js").to_str().unwrap())
        .env("PORT", listen_port_s)
        .env("HOST", config.clone().host)
        .current_dir(tmp.path())
        .stdout(stdout)
        .stderr(stderr)
        .output()
        .await
        .unwrap();

    println!(
        "(Client) STDOUT: {}",
        String::from_utf8(out.stdout).unwrap()
    );

    println!(
        "(Client) STDERR: {}",
        String::from_utf8(out.stderr).unwrap()
    );

    info("Exiting client process...");
}
