use std::sync::Arc;

use crate::cli::{ClientCli, ClientConfig};
use crate::secrets::{SecretData, SecretItemData, SecretLocationID, SecretRequest, SecretResponse};
use crate::service_request::{
    construct_memory_request, ClientCommand, DeviceData, ItemRequestData, ItemResponseData,
    SnesChannels, SnesMeta, SnesTxSlot, MEMORY_MAPPING, SNES_RX_COMMAND, SNES_RX_STATUS,
    SNES_SECRET_BUFFER, SNES_SECRET_ID_BUFFER, SNES_TX_STATUS,
};
use crate::sni::{device_memory_client::DeviceMemoryClient, devices_client::DevicesClient};
use crate::sni::{
    DevicesRequest, MultiReadMemoryRequest, MultiWriteMemoryRequest, ReadMemoryResponse,
    SingleWriteMemoryRequest, WriteMemoryRequest,
};

use clap::Parser;
use crossbeam_queue::ArrayQueue;
use hyper::body::Bytes;
use reqwest::Client;
use tokio::{sync::mpsc, time::sleep};

pub mod cli;
pub mod secrets;
pub mod service_request;
pub mod sni;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config: ClientConfig = ClientCli::parse().into_config();
    let (tx, rx) = mpsc::channel::<SecretRequest>(16);
    let q = Arc::new(ArrayQueue::<SecretResponse>::new(16));
    let handler_q = q.clone();
    let device_data = try_connect().await?;
    let mem_client = DeviceMemoryClient::connect("http://0.0.0.0:8191").await?;
    let mem_requests = construct_memory_request(device_data.clone());

    let mut client = ClientState::new(device_data, mem_client, tx.clone());

    let _handle = tokio::spawn(async move { secret_request_handler(config, rx, handler_q).await });

    loop {
        let (_meta, channel_data) =
            match read_from_snes(&mut client.snes_client, mem_requests.clone()).await {
                Some((m, d)) => (m, d),
                None => continue,
            };
        for i in 0..15 {
            match channel_data.transfer_status[i].pending() {
                false => (),
                true => match channel_data.tx_channel[i].command {
                    ClientCommand::Item => {
                        handle_item_request(i, &channel_data.tx_channel[i], &mut client).await?
                    }
                    ClientCommand::Acknowledge => acknowledge(&mut client).await?,
                    _ => (),
                },
            }
        }

        loop {
            match q.pop() {
                Some(r) => handle_item_receipt(r, &mut client).await?,
                None => break,
            };
        }
    }
}

struct ClientState {
    device: DeviceData,
    snes_client: DeviceMemoryClient<tonic::transport::Channel>,
    client_tx: mpsc::Sender<SecretRequest>,
    snes_tx_ptr: u16,
}

impl ClientState {
    fn new(
        device: DeviceData,
        snes_client: DeviceMemoryClient<tonic::transport::Channel>,
        tx: mpsc::Sender<SecretRequest>,
    ) -> Self {
        ClientState {
            device,
            snes_client,
            client_tx: tx,
            snes_tx_ptr: 0,
        }
    }
}

async fn try_connect() -> Result<DeviceData, Box<dyn std::error::Error>> {
    let mut device_client = loop {
        let maybe_client = match DevicesClient::connect("http://0.0.0.0:8191").await {
            Ok(d) => d,
            Err(_) => {
                sleep(std::time::Duration::from_secs(10)).await;
                continue;
            }
        };
        break maybe_client;
    };
    let device: DeviceData = loop {
        let maybe_device = match device_client.list_devices(DevicesRequest::default()).await {
            Ok(res) => match res.get_ref().devices.first() {
                Some(d) => DeviceData {
                    uri: d.uri.clone(),
                    address_space: d.default_address_space.into(),
                },
                None => {
                    sleep(std::time::Duration::from_secs(10)).await;
                    continue;
                }
            },
            Err(_) => {
                sleep(std::time::Duration::from_secs(10)).await;
                continue;
            }
        };
        break maybe_device;
    };

    Ok(device)
}

async fn secret_request_handler(
    config: ClientConfig,
    mut rx: mpsc::Receiver<SecretRequest>,
    q: Arc<ArrayQueue<SecretResponse>>,
) -> Result<(), reqwest::Error> {
    let client = Client::new();
    let addr = format!("http://{}", config.addr);
    loop {
        let req = match rx.recv().await {
            Some(r) => r,
            None => panic!("Fatal error: Secret request channel receiver"),
        };
        let req_bytes_array: [u8; 2] = match req {
            SecretRequest::Item(i) => i.location.into(),
            SecretRequest::Shop => continue, // not implemented,
        };
        let req_bytes = Bytes::copy_from_slice(req_bytes_array.as_slice());

        let http_res = client.get(&addr).body(req_bytes).send().await.unwrap();
        let secret_bytes: Bytes = http_res.bytes().await?;
        assert!(secret_bytes.len() == 2);
        let secret_data: SecretItemData = [secret_bytes[0], secret_bytes[1]].into();
        let snes_res = ItemResponseData {
            item: secret_data,
            location: req.location(),
            request_slot: req.request_slot(),
            secret_slot: req.secret_slot(),
        };

        q.push(SecretResponse::Item(snes_res))
            .expect("Error pushing secret response onto queue.");
    }
}

async fn read_from_snes(
    client: &mut DeviceMemoryClient<tonic::transport::Channel>,
    req: MultiReadMemoryRequest,
) -> Option<(SnesMeta, SnesChannels)> {
    let responses_data: Vec<ReadMemoryResponse> = match client.multi_read(req).await {
        Ok(m) => m.into_inner().responses,
        Err(_) => return None,
    };
    // If our marker isn't in WRAM then we can't use this memory
    if std::str::from_utf8(&responses_data[0].data).unwrap_or("") != "Echo Whistle        " {
        return None;
    }

    let meta_bytes = &responses_data[1].data;
    let meta = SnesMeta {
        rx_ptr: u16::from_le_bytes([meta_bytes[0], meta_bytes[1]]),
        tx_ptr: u16::from_le_bytes([meta_bytes[2], meta_bytes[3]]),
        connection_status: u16::from_le_bytes([meta_bytes[4], meta_bytes[5]]),
    };

    let channels = SnesChannels::from_slice(&responses_data[2].data);

    Some((meta, channels))
}

async fn handle_item_request(
    idx: usize,
    item_data: &SnesTxSlot,
    ctx: &mut ClientState,
) -> Result<(), Box<dyn std::error::Error>> {
    let offset: u32 = (idx as u32) * 2;
    let status_byte: Vec<u8> = vec![0x04, 0x00];
    let status_req = SingleWriteMemoryRequest {
        uri: ctx.device.uri.clone(),
        request: Some(WriteMemoryRequest {
            request_address: SNES_TX_STATUS + offset,
            request_address_space: ctx.device.address_space as i32,
            request_memory_mapping: MEMORY_MAPPING as i32,
            data: status_byte,
        }),
    };
    ctx.snes_client.single_write(status_req).await?;

    let item_req = ItemRequestData {
        location: SecretLocationID::from(item_data.data as u16),
        request_slot: idx as u16,
        secret_slot: (item_data.data >> 16) as u16,
    };

    let secret_req = SecretRequest::Item(item_req);
    ctx.client_tx.send(secret_req).await?;
    Ok(())
}

async fn handle_item_receipt(
    item: SecretResponse,
    ctx: &mut ClientState,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut write_vec: Vec<WriteMemoryRequest> = Vec::with_capacity(3);

    let secret_bytes = item.secret_bytes().to_le_bytes().to_vec();
    let location_bytes = u16::from(item.location()).to_le_bytes().to_vec();
    let status_bytes: Vec<u8> = vec![0x03, 0x00];
    let secret_write = WriteMemoryRequest {
        request_address: SNES_SECRET_BUFFER + (item.secret_slot() * 2) as u32,
        request_address_space: ctx.device.address_space as i32,
        request_memory_mapping: MEMORY_MAPPING as i32,
        data: secret_bytes,
    };
    let location_write = WriteMemoryRequest {
        request_address: SNES_SECRET_ID_BUFFER + (item.secret_slot() * 2) as u32,
        request_address_space: ctx.device.address_space as i32,
        request_memory_mapping: MEMORY_MAPPING as i32,
        data: location_bytes,
    };
    let status_write = WriteMemoryRequest {
        request_address: SNES_TX_STATUS + (item.secret_slot() * 2) as u32,
        request_address_space: ctx.device.address_space as i32,
        request_memory_mapping: MEMORY_MAPPING as i32,
        data: status_bytes,
    };

    write_vec.push(secret_write);
    write_vec.push(location_write);
    write_vec.push(status_write);

    let multi_write = MultiWriteMemoryRequest {
        uri: ctx.device.uri.clone(),
        requests: write_vec,
    };
    ctx.snes_client.multi_write(multi_write).await?;

    Ok(())
}

async fn acknowledge(ctx: &mut ClientState) -> Result<(), Box<dyn std::error::Error>> {
    let command_bytes: Vec<u8> = vec![0x00, 0x02, 0x00, 0x00];
    let command_req = SingleWriteMemoryRequest {
        uri: ctx.device.uri.clone(),
        request: Some(WriteMemoryRequest {
            request_address: SNES_RX_COMMAND + ctx.snes_tx_ptr as u32,
            request_address_space: ctx.device.address_space as i32,
            request_memory_mapping: MEMORY_MAPPING as i32,
            data: command_bytes,
        }),
    };

    let status_bytes: Vec<u8> = vec![0x00, 0x08];
    let status_req = SingleWriteMemoryRequest {
        uri: ctx.device.uri.clone(),
        request: Some(WriteMemoryRequest {
            request_address: SNES_RX_STATUS + (ctx.snes_tx_ptr as u32),
            request_address_space: ctx.device.address_space as i32,
            request_memory_mapping: MEMORY_MAPPING as i32,
            data: status_bytes,
        }),
    };

    ctx.snes_tx_ptr = (ctx.snes_tx_ptr + 2) & 0x1F;

    ctx.snes_client.single_write(command_req).await?;
    ctx.snes_client.single_write(status_req).await?;

    Ok(())
}
