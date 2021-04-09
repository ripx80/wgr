use std::env;
use std::fs::File;
use std::process::exit;

// clap cmd line parser
// slog? json logger, ggf with env vars for human output
// tokio? async and fast
// look at thiserror(for libs), anyhow(for apps) libs for error handling
// serde for json and yaml
// validator to validate json and yaml
// sodiumoxide?
/*


TODO:

- json deserilize/serialize:
    - convert base64 strings to internal curve keys (pub,priv,psk)
    - convert ip strings to ipv4/6


- error handling
    - use save exit not panic
    - errors to json

- crypto
    - use x25519_dalek::{PublicKey, StaticSecret};
    - use base64

via netlink
- create wg interface
- config wg interface
- delete wg interface

*/

mod wireguard;

//use anyhow::Result;
use std::collections::HashMap;
use wireguard::types::Device;

extern crate serde_derive;
extern crate serde_json;

type Config = HashMap<String, Device>;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("open file error {0}")]
    FileError(#[from] std::io::Error),
    #[error("invalid json error {0}")]
    Json(#[from] serde_json::Error),
    // #[error("invalid key {0}")]
    // Key(#[from] serde_json::Error),
}

fn read_config() -> std::result::Result<Config, ConfigError> {
    let f = File::open("wg.json")?;
    Ok(serde_json::from_reader(f)?)
}

fn main() -> std::result::Result<(), ConfigError> {
    let mut name = None;
    let mut args = env::args();

    args.next();
    for arg in args {
        match arg.as_str() {
            dev => name = Some(dev.to_owned()),
        }
    }
    // unwrap device name
    let name = match name {
        None => {
            eprintln!("No device name supplied");
            exit(-1);
        }
        Some(name) => name,
    };

    // read config
    let conf = match read_config() {
        Ok(conf) => conf,
        Err(error) => return Err(error),
    };
    println!("{}", serde_json::to_string_pretty(&conf)?);

    println!("create dev: {}", name);
    Ok(())
}
