use crate::service_request::{ItemRequestData, ItemResponseData};

use serde::{Deserialize, Serialize};

pub trait SecretData {
    // TODO: this should be arbitrary &[u8] or allocated container for potential non-items
    fn secret_bytes(&self) -> u16;

    fn request_slot(&self) -> u16;

    fn secret_slot(&self) -> u16;

    fn location(&self) -> SecretLocationID;
}

#[derive(Debug, Copy, Clone)]
pub enum SecretRequest {
    Item(ItemRequestData),
    Shop,
}

#[derive(Debug, Copy, Clone)]
pub enum SecretResponse {
    Item(ItemResponseData),
    Shop,
}

impl SecretData for SecretRequest {
    fn secret_bytes(&self) -> u16 {
        match self {
            SecretRequest::Item(i) => u16::from_be_bytes(<[u8; 2]>::from(i.location)),
            SecretRequest::Shop => 0xFFFFu16,
        }
    }
    fn request_slot(&self) -> u16 {
        match self {
            SecretRequest::Item(i) => i.request_slot,
            SecretRequest::Shop => 0xFFFFu16,
        }
    }
    fn secret_slot(&self) -> u16 {
        match self {
            SecretRequest::Item(i) => i.secret_slot,
            SecretRequest::Shop => 0xFFFFu16,
        }
    }
    fn location(&self) -> SecretLocationID {
        match self {
            SecretRequest::Item(i) => i.location,
            SecretRequest::Shop => SecretLocationID::Antlion,
        }
    }
}

impl SecretData for SecretResponse {
    fn secret_bytes(&self) -> u16 {
        match self {
            SecretResponse::Item(i) => u16::from_be_bytes(<[u8; 2]>::from(i.item)),
            SecretResponse::Shop => 0xFFFFu16,
        }
    }
    fn request_slot(&self) -> u16 {
        match self {
            SecretResponse::Item(i) => i.request_slot,
            SecretResponse::Shop => 0xFFFFu16,
        }
    }
    fn secret_slot(&self) -> u16 {
        match self {
            SecretResponse::Item(i) => i.secret_slot,
            SecretResponse::Shop => 0xFFFFu16,
        }
    }
    fn location(&self) -> SecretLocationID {
        match self {
            SecretResponse::Item(i) => i.location,
            SecretResponse::Shop => SecretLocationID::Antlion,
        }
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Default)]
#[repr(u16)]
pub enum SecretItemData {
    #[default]
    None,
    #[serde(alias = "boreas")]
    Boreas = 0x00B3,
    #[serde(alias = "gugnir")]
    Gugnir = 0x002A,
    #[serde(alias = "crossbow")]
    Crossbow = 0x004E,
    #[serde(alias = "whistle")]
    Whistle = 0x00ED,
    #[serde(alias = "luca key")]
    LucaKey = 0x02F3,
    #[serde(alias = "hook")]
    Hook = 0x02FC,
}

impl From<[u8; 2]> for SecretItemData {
    fn from(v: [u8; 2]) -> Self {
        let discriminant = u16::from_be_bytes(v);

        match discriminant {
            0x002A => SecretItemData::Gugnir,
            0x004E => SecretItemData::Crossbow,
            0x00B3 => SecretItemData::Boreas,
            0x00ED => SecretItemData::Whistle,
            0x02F3 => SecretItemData::LucaKey,
            0x02FC => SecretItemData::Hook,
            // TODO: In a full service we should handle errors here and not assume the data that
            // the client and server are receiving is properly formed.
            _ => std::panic!("Could not convert bytes to SecretItemData"),
        }
    }
}

impl From<SecretItemData> for [u8; 2] {
    fn from(v: SecretItemData) -> Self {
        (v as u16).to_be_bytes()
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub enum SecretLocationID {
    Antlion,
    Fabul,
    Ordeals,
    BaronInn,
    ToroiaCastle,
    Starting,
}

impl From<[u8; 2]> for SecretLocationID {
    fn from(v: [u8; 2]) -> Self {
        let discriminant = u16::from_be_bytes(v);

        match discriminant {
            0x0000 => SecretLocationID::Antlion,
            0x0001 => SecretLocationID::Fabul,
            0x0002 => SecretLocationID::Ordeals,
            0x0003 => SecretLocationID::BaronInn,
            0x0004 => SecretLocationID::ToroiaCastle,
            0x0005 => SecretLocationID::Starting,
            _ => std::panic!("Could not convert bytes to SecretLocationID"),
        }
    }
}

impl From<u16> for SecretLocationID {
    fn from(v: u16) -> Self {
        match v {
            0x0000 => SecretLocationID::Antlion,
            0x0001 => SecretLocationID::Fabul,
            0x0002 => SecretLocationID::Ordeals,
            0x0003 => SecretLocationID::BaronInn,
            0x0004 => SecretLocationID::ToroiaCastle,
            0x0005 => SecretLocationID::Starting,
            _ => std::panic!("Could not convert bytes to SecretLocationID"),
        }
    }
}

impl From<SecretLocationID> for [u8; 2] {
    fn from(v: SecretLocationID) -> Self {
        (v as u16).to_be_bytes()
    }
}

impl From<SecretLocationID> for u16 {
    fn from(v: SecretLocationID) -> Self {
        v as u16
    }
}
