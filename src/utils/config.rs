use serde::Deserialize;

#[derive(Deserialize)]
pub struct HostServer {
    pub guild_id: u64,
    pub verify_channel: u64,
    pub roles: Vec<u64>,
}

#[derive(Deserialize)]
pub struct Config {
    pub owner: u64,
    pub host_server: HostServer,
    pub roomba_allowed_servers: Vec<u64>,
}
