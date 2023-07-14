use std::net::{SocketAddr, SocketAddrV4};

use lazy_static::lazy_static;
use serde::Deserialize;

use crate::errors::R;

#[derive(Deserialize)]
pub struct EnvVars {
    pub host: String,
    pub port: u16,
    pub log_level: String,
    pub database_url: String,
    pub username: String,
    pub password: String,
}

impl EnvVars {
    pub fn get_addr(&self) -> R<SocketAddr> {
        let addr_v4 = SocketAddrV4::new(self.host.parse()?, self.port);

        Ok(SocketAddr::V4(addr_v4))
    }
}

lazy_static! {
    pub static ref ENV_VARS: EnvVars = envy::from_env::<EnvVars>().unwrap();
}
