use std::ops::Index;

use clap::Parser;
use hyper::service::{make_service_fn, service_fn};
use hyper::{body::Bytes, Body, Request, Response, Server};
use once_cell::sync::OnceCell;
use serde::Deserialize;

use crate::{
    cli::{ServerCli, ServerConfig},
    secrets::{SecretItemData, SecretLocationID},
};

pub mod cli;
pub mod secrets;
pub mod service_request;
pub mod sni;

static SECRETS_ARRAY: OnceCell<SecretsArray<6>> = OnceCell::new();

#[derive(Debug, Copy, Clone, Deserialize)]
struct SecretsYaml {
    antlion_cave: SecretItemData,
    fabul_castle: SecretItemData,
    ordeals: SecretItemData,
    baron_inn: SecretItemData,
    toroia_castle: SecretItemData,
    starting: SecretItemData,
}

impl SecretsYaml {
    fn into_array(self) -> SecretsArray<6> {
        // TODO: sort secrets by index on construction
        [
            self.antlion_cave,
            self.fabul_castle,
            self.ordeals,
            self.baron_inn,
            self.toroia_castle,
            self.starting, // Extremely silly oversight on my part
        ]
    }
}

// We use const generics here but in a fully-featured service we could
// have a static number of secrets.
pub type SecretsArray<const N: usize> = [SecretItemData; N];

impl<const N: usize> Index<SecretLocationID> for SecretsArray<N> {
    type Output = SecretItemData;

    fn index(&self, id: SecretLocationID) -> &Self::Output {
        &self[id as usize]
    }
}

async fn process(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let body_bytes = hyper::body::to_bytes(req.into_body()).await?;
    assert!(
        body_bytes.len() == 2,
        "Server received malformed secret request."
    );
    let secret_request: SecretLocationID = [body_bytes[0], body_bytes[1]].into();
    let secret_data: [u8; 2] = SECRETS_ARRAY.get().unwrap()[secret_request].into();
    let res_bytes: Bytes = Bytes::copy_from_slice(secret_data.as_slice());

    Ok(Response::new(res_bytes.into()))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let config: ServerConfig = ServerCli::parse().into_config();
    let yaml_file = std::fs::read_to_string(config.yaml_path).expect("Failed reading YAML file.");
    let secrets: SecretsYaml = serde_yaml::from_str(&yaml_file).expect("Failed parsing YAML file");
    SECRETS_ARRAY
        .set(secrets.into_array())
        .expect("Failed initializing secrets array constant.");
    let make_service = make_service_fn(|_| async { Ok::<_, hyper::Error>(service_fn(process)) });

    let server = Server::bind(&config.addr).serve(make_service);

    println!("Listening on http://{}", &config.addr);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }

    Ok(())
}
