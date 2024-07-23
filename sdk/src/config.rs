use core::str::FromStr;
use std::collections::{BTreeMap, HashMap};
use std::fs::create_dir_all;
use std::net::IpAddr;
use std::path::PathBuf;

use crate::OxideError;
use serde::{Deserialize, Serialize};
// use toml_edit::{Item, Table};
// use uuid::Uuid;

/*

[endpoints]

[endpoints."ovh-eu"]
client_id = ""
client_secret = ""

*/

#[derive(Debug)]
pub struct Config {
    pub client_id: String,
    pub client_secret: String,
    pub endpoint: String,
}

impl Config {
    pub fn new_with_credentials(endpoint: String, client_id: String, client_secret: String) -> Self {
        Self {
            endpoint,
            client_id,
            client_secret
        }
    }
}