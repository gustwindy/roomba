use serde::Deserialize;

#[derive(Deserialize)]
pub struct HostServer {
    pub guild_id: u64,
    pub verify_channel: u64,
    pub verify_msg: u64,
    pub roles: Vec<u64>,

    pub welcome: u64,
    pub join_logs: u64,
}

#[derive(Deserialize)]
pub struct FindConfig {
    pub limit: i32,
    pub refetch_ratio: f32,
    pub yuri_tags: Vec<String>,
}

#[derive(Deserialize)]
pub struct CommandsConfig {
    pub find: FindConfig,
}

#[derive(Deserialize)]
pub struct Config {
    pub owner: u64,
    pub roomba_allowed_servers: Vec<u64>,

    pub host_server: HostServer,
    pub commands: CommandsConfig,
}
