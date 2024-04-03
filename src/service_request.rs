use crate::{
    secrets::{SecretItemData, SecretLocationID},
    sni::{AddressSpace, MemoryMapping, MultiReadMemoryRequest, ReadMemoryRequest},
};

pub const MEMORY_MAPPING: MemoryMapping = MemoryMapping::LoRom;
pub const SNES_META: u32 = 0x7E16C0;
pub const SNES_SECRET_BUFFER: u32 = 0x7FA000;
pub const SNES_SECRET_ID_BUFFER: u32 = 0x7FA020;
pub const SNES_RX_COMMAND: u32 = 0x7FA040;
pub const SNES_TX_COMMAND: u32 = 0x7FA060;
pub const SNES_RX_ARGS: u32 = 0x7FA080;
pub const SNES_TX_ARGS: u32 = 0x7FA0C0;
pub const SNES_RX_STATUS: u32 = 0x7FA100;
pub const SNES_TX_STATUS: u32 = 0x7FA120;
pub const SNES_MARKER: u32 = 0x7FA140;

#[derive(Debug, Copy, Clone)]
pub struct ItemRequestData {
    pub location: SecretLocationID,
    pub request_slot: u16,
    pub secret_slot: u16,
}

#[derive(Debug, Copy, Clone)]
pub struct ItemResponseData {
    pub item: SecretItemData,
    pub location: SecretLocationID,
    pub request_slot: u16,
    pub secret_slot: u16,
}

#[derive(Debug, Clone)]
pub struct DeviceData {
    pub uri: String,
    pub address_space: AddressSpace,
}

impl From<i32> for AddressSpace {
    fn from(n: i32) -> Self {
        match n {
            0 => AddressSpace::FxPakPro,
            1 => AddressSpace::SnesABus,
            2 => AddressSpace::Raw,
            _ => AddressSpace::SnesABus,
        }
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct SnesRequestStatus(u16);

impl SnesRequestStatus {
    pub fn from_slice(b: &[u8]) -> Self {
        SnesRequestStatus(u16::from_le_bytes([b[0], b[1]]))
    }

    pub fn open(&self) -> bool {
        (self.0 & 0x01) != 0
    }

    pub fn success(&self) -> bool {
        (self.0 & 0x02) != 0
    }

    pub fn failure(&self) -> bool {
        (self.0 & 0x80) != 0
    }

    pub fn pending(&self) -> bool {
        (self.0 & 0x08) != 0
    }

    pub fn acknowledged(&self) -> bool {
        (self.0 & 0x04) != 0
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct SnesMeta {
    pub rx_ptr: u16,
    pub tx_ptr: u16,
    pub connection_status: u16,
}

#[derive(Debug, Copy, Clone, Default)]
pub struct SnesRxSlot {
    pub command: SnesCommand,
    pub data: u32,
}

impl SnesRxSlot {
    pub fn from_snes_buffer(buf: &[u8], idx: usize) -> Self {
        SnesRxSlot {
            command: SnesCommand::from_le_bytes([buf[idx * 2], buf[(idx * 2) + 1]]),
            data: u32::from_le_bytes([
                buf[0x40 + (idx * 4)],
                buf[0x40 + (idx * 4) + 1],
                buf[0x40 + (idx * 4) + 2],
                buf[0x40 + (idx * 4) + 3],
            ]),
        }
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct SnesTxSlot {
    pub command: ClientCommand,
    pub data: u32,
}

impl SnesTxSlot {
    pub fn from_snes_buffer(buf: &[u8], idx: usize) -> Self {
        SnesTxSlot {
            command: ClientCommand::from_le_bytes([
                buf[0x20 + (idx * 2)],
                buf[0x20 + (idx * 2) + 1],
            ]),
            data: u32::from_le_bytes([
                buf[0x80 + (idx * 4)],
                buf[0x80 + (idx * 4) + 1],
                buf[0x80 + (idx * 4) + 2],
                buf[0x80 + (idx * 4) + 3],
            ]),
        }
    }
}

#[derive(Debug, Copy, Clone, Default)]
#[repr(u16)]
#[non_exhaustive]
pub enum SnesCommand {
    // Command our client can send to the SNES
    #[default]
    None,
    ClearSlot,
    ConfirmConnection,
}

impl SnesCommand {
    pub fn from_le_bytes(b: [u8; 2]) -> Self {
        match u16::from_le_bytes([b[0], b[1]]) {
            0 => SnesCommand::None,
            1 => SnesCommand::ClearSlot,
            2 => SnesCommand::ConfirmConnection,
            _ => SnesCommand::None,
        }
    }
}

impl From<u16> for SnesCommand {
    fn from(v: u16) -> Self {
        match v {
            0 => SnesCommand::None,
            1 => SnesCommand::ClearSlot,
            2 => SnesCommand::ConfirmConnection,
            _ => SnesCommand::None,
        }
    }
}

#[derive(Debug, Copy, Clone, Default)]
#[repr(u16)]
#[non_exhaustive]
pub enum ClientCommand {
    // Command the SNES can send to our client
    #[default]
    None,
    Item,
    Acknowledge,
}

impl ClientCommand {
    pub fn from_le_bytes(b: [u8; 2]) -> Self {
        match u16::from_le_bytes([b[0], b[1]]) {
            0 => ClientCommand::None,
            1 => ClientCommand::Item,
            2 => ClientCommand::Acknowledge,
            _ => ClientCommand::None,
        }
    }
}

impl From<u16> for ClientCommand {
    fn from(v: u16) -> Self {
        match v {
            0 => ClientCommand::None,
            1 => ClientCommand::Item,
            2 => ClientCommand::Acknowledge,
            _ => ClientCommand::None,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct SnesChannels {
    pub rx_channel: [SnesRxSlot; 0x10],
    pub tx_channel: [SnesTxSlot; 0x10],
    pub request_status: [SnesRequestStatus; 0x10],
    pub transfer_status: [SnesRequestStatus; 0x10],
}

impl SnesChannels {
    pub fn from_slice(b: &[u8]) -> Self {
        let rx_channel = [
            SnesRxSlot::from_snes_buffer(b, 0),
            SnesRxSlot::from_snes_buffer(b, 1),
            SnesRxSlot::from_snes_buffer(b, 2),
            SnesRxSlot::from_snes_buffer(b, 3),
            SnesRxSlot::from_snes_buffer(b, 4),
            SnesRxSlot::from_snes_buffer(b, 5),
            SnesRxSlot::from_snes_buffer(b, 6),
            SnesRxSlot::from_snes_buffer(b, 7),
            SnesRxSlot::from_snes_buffer(b, 8),
            SnesRxSlot::from_snes_buffer(b, 9),
            SnesRxSlot::from_snes_buffer(b, 10),
            SnesRxSlot::from_snes_buffer(b, 11),
            SnesRxSlot::from_snes_buffer(b, 12),
            SnesRxSlot::from_snes_buffer(b, 13),
            SnesRxSlot::from_snes_buffer(b, 14),
            SnesRxSlot::from_snes_buffer(b, 15),
        ];
        let tx_channel = [
            SnesTxSlot::from_snes_buffer(b, 0),
            SnesTxSlot::from_snes_buffer(b, 1),
            SnesTxSlot::from_snes_buffer(b, 2),
            SnesTxSlot::from_snes_buffer(b, 3),
            SnesTxSlot::from_snes_buffer(b, 4),
            SnesTxSlot::from_snes_buffer(b, 5),
            SnesTxSlot::from_snes_buffer(b, 6),
            SnesTxSlot::from_snes_buffer(b, 7),
            SnesTxSlot::from_snes_buffer(b, 8),
            SnesTxSlot::from_snes_buffer(b, 9),
            SnesTxSlot::from_snes_buffer(b, 10),
            SnesTxSlot::from_snes_buffer(b, 11),
            SnesTxSlot::from_snes_buffer(b, 12),
            SnesTxSlot::from_snes_buffer(b, 13),
            SnesTxSlot::from_snes_buffer(b, 14),
            SnesTxSlot::from_snes_buffer(b, 15),
        ];
        let request_status = [
            SnesRequestStatus::from_slice(&b[0xC0..0xC2]),
            SnesRequestStatus::from_slice(&b[0xC2..0xC4]),
            SnesRequestStatus::from_slice(&b[0xC4..0xC6]),
            SnesRequestStatus::from_slice(&b[0xC6..0xC8]),
            SnesRequestStatus::from_slice(&b[0xC8..0xCA]),
            SnesRequestStatus::from_slice(&b[0xCA..0xCC]),
            SnesRequestStatus::from_slice(&b[0xCC..0xCE]),
            SnesRequestStatus::from_slice(&b[0xCE..0xD0]),
            SnesRequestStatus::from_slice(&b[0xD0..0xD2]),
            SnesRequestStatus::from_slice(&b[0xD2..0xD4]),
            SnesRequestStatus::from_slice(&b[0xD4..0xD6]),
            SnesRequestStatus::from_slice(&b[0xD6..0xD8]),
            SnesRequestStatus::from_slice(&b[0xD8..0xDA]),
            SnesRequestStatus::from_slice(&b[0xDA..0xDC]),
            SnesRequestStatus::from_slice(&b[0xDC..0xDE]),
            SnesRequestStatus::from_slice(&b[0xDE..0xE0]),
        ];
        let transfer_status = [
            SnesRequestStatus::from_slice(&b[0xE0..0xE2]),
            SnesRequestStatus::from_slice(&b[0xE2..0xE4]),
            SnesRequestStatus::from_slice(&b[0xE4..0xE6]),
            SnesRequestStatus::from_slice(&b[0xE6..0xE8]),
            SnesRequestStatus::from_slice(&b[0xE8..0xEA]),
            SnesRequestStatus::from_slice(&b[0xEA..0xEC]),
            SnesRequestStatus::from_slice(&b[0xEC..0xEE]),
            SnesRequestStatus::from_slice(&b[0xEE..0xF0]),
            SnesRequestStatus::from_slice(&b[0xF0..0xF2]),
            SnesRequestStatus::from_slice(&b[0xF2..0xF4]),
            SnesRequestStatus::from_slice(&b[0xF4..0xF6]),
            SnesRequestStatus::from_slice(&b[0xF6..0xF8]),
            SnesRequestStatus::from_slice(&b[0xF8..0xFA]),
            SnesRequestStatus::from_slice(&b[0xFA..0xFC]),
            SnesRequestStatus::from_slice(&b[0xFC..0xFE]),
            SnesRequestStatus::from_slice(&b[0xFE..0x100]),
        ];
        SnesChannels {
            rx_channel,
            tx_channel,
            request_status,
            transfer_status,
        }
    }
}

pub fn construct_memory_request(d: DeviceData) -> MultiReadMemoryRequest {
    let mut req_vec: Vec<ReadMemoryRequest> = Vec::with_capacity(2);
    let marker_req = ReadMemoryRequest {
        request_address: SNES_MARKER,
        request_address_space: d.address_space as i32,
        request_memory_mapping: MEMORY_MAPPING as i32,
        size: 0x14,
    };
    let meta_req = ReadMemoryRequest {
        request_address: SNES_META,
        request_address_space: d.address_space as i32,
        request_memory_mapping: MEMORY_MAPPING as i32,
        size: 0x06,
    };
    let channel_req = ReadMemoryRequest {
        request_address: SNES_RX_COMMAND,
        request_address_space: d.address_space as i32,
        request_memory_mapping: MEMORY_MAPPING as i32,
        size: 0x100,
    };
    req_vec.push(marker_req);
    req_vec.push(meta_req);
    req_vec.push(channel_req);

    MultiReadMemoryRequest {
        uri: d.uri,
        requests: req_vec,
    }
}
