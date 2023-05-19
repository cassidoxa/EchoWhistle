use std::{
    net::{IpAddr, SocketAddr},
    path::PathBuf,
};

use clap::Parser;

#[derive(Parser)]
#[command(author, version)]
pub struct ClientCli {
    #[arg(long)]
    host: Option<String>,
    port: Option<u16>,
}

impl ClientCli {
    pub fn into_config(self) -> ClientConfig {
        let host: IpAddr = self
            .host
            .unwrap_or("127.0.0.1".to_string())
            .parse()
            .expect("Error parsing IP address.");

        let port: u16 = self.port.unwrap_or(38281);
        let addr = SocketAddr::new(host, port);

        ClientConfig { addr }
    }
}

pub struct ClientConfig {
    pub addr: SocketAddr,
}

#[derive(Parser)]
#[command(author, version)]
pub struct ServerCli {
    #[arg(long)]
    host: Option<String>,
    port: Option<u16>,
    secret_yaml: Option<String>,
}

impl ServerCli {
    pub fn into_config(self) -> ServerConfig {
        let host: IpAddr = self
            .host
            .unwrap_or("127.0.0.1".to_string())
            .parse()
            .expect("Error parsing IP address.");

        let port: u16 = self.port.unwrap_or(38281);
        let addr = SocketAddr::new(host, port);

        let maybe_yaml_path: String = self.secret_yaml.unwrap_or("./secrets_ki.yaml".to_string());
        let yaml_path = PathBuf::from(maybe_yaml_path);

        ServerConfig { addr, yaml_path }
    }
}

pub struct ServerConfig {
    pub addr: SocketAddr,
    pub yaml_path: PathBuf,
}
