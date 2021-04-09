// #[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct Config {
//     pub name: HashMap<String, Device>,
// }

extern crate base64;

//use hex::FromHex;
use serde::de;
use std::collections::HashMap;
//use std::convert::TryInto;
use thiserror::Error;
//use x25519_dalek::{PublicKey, StaticSecret};

#[derive(Error, Debug)]
pub enum KeyError {
    #[error("open file error {0}")]
    FileError(#[from] std::io::Error),
}

fn from_base64<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: de::Deserializer<'de>,
{
    let s: &str = de::Deserialize::deserialize(deserializer)?;
    serde_json::from_str(&s).map_err(de::Error::custom)

    //println!("{:#?}", v);
    //let mut d = base64::decode(v).map_err(de::Error::custom)?;
    //let d :Vec<u8> = Vec<2,3,4,5>
    // let key: Key = [
    //     1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
    //     1, 1,
    // ];
    //let key = <[u8; 32]>::from_hex(v).map_err(de::Error::custom)?;
    //Ok(key)

    //Ok(key)

    //Vec<u8> to [u8,32]

    //let base64_config = base64::Config::new(base64::CharacterSet::UrlSafe, true);
}
// fn to_byte<T>(v: Vec<T>) -> [T; 32] {
//     match v.try_into().unwrap_or_else(|v: Vec<T>| {
//         panic!("Expected a Vec of length {} but it was {}", 32, v.len())
//     })
// }

pub type Key = [u8; 32];
pub type Vkey = Vec<u8>;

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Device {
    #[serde(deserialize_with = "from_base64")]
    //#[serde(deserialize_with = "ser_base64")]
    pub private_key: String,
    //pub private_key: Key,
    pub public_key: String,
    pub peers: HashMap<String, Peer>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub listen_port: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub devtype: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fwmark: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub table: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Peer {
    pub endpoint: String,
    pub allowed_ips: Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub psk: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub keepalive: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<i64>,
}
