use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use bft_core::{InternalBFTClient, IncomingMsgsStorage, get_monotonic_time};
use bft_core::messages::{ClientRequestMsg, MessageBase};
use bft_core::client::{ClientId, Client};
use bft_core::client_request_msg::ClientRequestMsg;
use bft_core::messages::client_request_msg::ClientRequestMsg;
use crate::config::Config;
use crate::config::ConfigBuilder;
use crate::config::ConfigBuilderError;
use crate::config::ConfigError;
use crate::config::ConfigParseError;
use crate::config::ConfigType;
use crate::config::ConfigTypeBuilder;
use crate::config::ConfigTypeBuilderError;
use crate::config::ConfigTypeError;
use crate::config::ConfigTypeParseError;
use crate::config::ConfigTypeValue;


fn main() {
    println!("Number of keys to generate? ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input: u32 = input.trim().parse().unwrap();
    let mut keys = Vec::new();
    for _ in 0..input {
        let (public_key, private_key) = rsa::generate_keypair(2048);
        keys.push((public_key, private_key));
    }
    println!("Filename for public keys? ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let filename = input.trim();
    let mut file = File::create(filename).unwrap();
    let mut map = Map::new();
    for (i, key) in keys.iter().enumerate() {
        map.insert(i as u32, key.0);
    }
    let map = map;
    let encoded: String = bincode::serialize(&map).unwrap();
    file.write_all(encoded.as_bytes()).unwrap();
    println!("Folder for private keys? ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let dirname = input.trim();
    fs::create_dir(dirname).unwrap();
    for (i, key) in keys.iter().enumerate() {
        let filename = format!("{}/{}.txt", dirname, i);
        let mut file = File::create(filename).unwrap();
        let encoded: String = bincode::serialize(&key.1).unwrap();
        file.write_all(encoded.as_bytes()).unwrap();
    }
}

