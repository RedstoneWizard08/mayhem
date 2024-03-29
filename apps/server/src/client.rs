// ============== !!! WARNING !!! ==============
// There is a Bun.js module for this which can
// run the server but it is very much a work-in-
// progress feature! DO NOT USE IT YET! It is not
// in a state where it will work yet!
// ============== !!! WARNING !!! ==============

use flate2::read::GzDecoder;
use indicatif::{ProgressBar, ProgressStyle};
use std::{
    cmp::min,
    env::{
        self,
        consts::{ARCH, OS},
    },
    fs::{self, File},
    io::{copy, Cursor, Read, Write},
    net::SocketAddr,
    path::Path,
    process::Stdio,
};
use tar::Archive;
use tokio::process::Command;
use tokio_stream::StreamExt;
use zip_extract::extract;

use include_dir::{include_dir, Dir};
use tempdir::TempDir;

use crate::{config::get_config, logging::info, util::parse_ip};

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

pub fn get_bun_download_url() -> String {
    let base = "https://github.com/oven-sh/bun/releases/download/bun-v0.6.1/bun-";
    let mut arch = "";

    if OS == "linux" {
        if ARCH == "x86" || ARCH == "x86_64" {
            arch = "linux-x64";
        }

        if ARCH == "arm" || ARCH == "aarch64" {
            arch = "linux-aarch64";
        }
    } else if OS == "macos" {
        if ARCH == "x86" || ARCH == "x86_64" {
            arch = "darwin-x64";
        }

        if ARCH == "arm" || ARCH == "aarch64" {
            arch = "darwin-aarch64";
        }
    }

    return format!("{}{}.zip", base, arch);
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

pub fn get_bun_dir_name() -> String {
    let base = "bun-";
    let mut arch = "";

    if OS == "linux" {
        if ARCH == "x86" || ARCH == "x86_64" {
            arch = "linux-x64";
        }

        if ARCH == "arm" || ARCH == "aarch64" {
            arch = "linux-aarch64";
        }
    } else if OS == "macos" {
        if ARCH == "x86" || ARCH == "x86_64" {
            arch = "darwin-x64";
        }

        if ARCH == "arm" || ARCH == "aarch64" {
            arch = "darwin-aarch64";
        }
    }

    return format!("{}{}", base, arch);
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

pub async fn download_bun(dir: &Path) {
    let url = get_bun_download_url();

    info("Downloading Bun...");

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

    let dl_dest_file = dir.join("bun.zip");

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

    info("Setting up Bun install...");

    let mut in_file = File::open(dl_dest_file).unwrap();
    let mut in_bytes = Vec::new();

    in_file.read_to_end(&mut in_bytes);

    info("Extracting Bun...");

    extract(Cursor::new(in_bytes), &dir, true).unwrap();
}

pub async fn run_client_node() {
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

    info("Installing moment.js and axios...");

    let node_path = tmp.path().clone().join("node");
    let node_path = node_path.to_str().unwrap().to_string();
    let modified_path = node_path + "/bin:" + &env::var("PATH").unwrap();

    let install = Command::new("npm")
        .arg("install")
        .arg("moment")
        .arg("axios")
        .env("PATH", &modified_path)
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

    let out = Command::new("node")
        .arg(tmp.path().join("index.js").to_str().unwrap())
        .env("PORT", listen_port_s)
        .env("HOST", config.clone().host)
        .env("PATH", &modified_path)
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

pub async fn run_client_bun() {
    let tmp = TempDir::new("mayhem").unwrap();

    download_bun(tmp.path()).await;

    info("Extracting frontend files...");

    CLIENT_DIR.extract(tmp.path());

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

    info("Installing dependencies...");

    let bun_path = tmp.path().clone().join("bun");
    let bun_path = bun_path.to_str().unwrap().to_string();
    let modified_path = bun_path + ":" + &env::var("PATH").unwrap();

    let install = Command::new("bun")
        .arg("install")
        .arg("cookie")
        .arg("set-cookie-parser")
        .arg("devalue")
        .env("PATH", &modified_path)
        .current_dir(tmp.path())
        .stdout(stdout)
        .stderr(stderr)
        .output()
        .await
        .unwrap();

    println!(
        "(BUN) STDOUT: {}",
        String::from_utf8(install.stdout).unwrap()
    );

    println!(
        "(BUN) STDERR: {}",
        String::from_utf8(install.stderr).unwrap()
    );

    let file_stdout = File::create(tmp.path().join("stdout.log")).unwrap();
    let file_stderr = File::create(tmp.path().join("stderr.log")).unwrap();

    let stdout = Stdio::from(file_stdout);
    let stderr = Stdio::from(file_stderr);

    info(format!("Client listening on {}", address).as_str());

    let out = Command::new("bun")
        .arg(tmp.path().join("index.js").to_str().unwrap())
        .env("PORT", listen_port_s)
        .env("HOST", config.clone().host)
        .env("PATH", &modified_path)
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
