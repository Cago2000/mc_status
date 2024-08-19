use crate::{config, output, Config};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Clone, clap::Args)]
pub struct Server {
    pub host: String,
    pub port: u16,
    pub alias: String
}

#[derive(clap::Args, Clone)]
pub struct Cli {
    #[clap(flatten)]
    pub server: Server,
}

impl Cli {
    pub async fn run(self) -> Result<()> {
        let mut config = Config::load()?;
        let server = config::Server::new(self.server.host.clone(), self.server.port, self.server.alias);
        config.server_list.insert(server.clone());
        let server_status = crate::get_server_status(&self.server.host, self.server.port).await
            .context("failed to get server status");
        match server_status {
            Ok(server_status) => {
                output::display_response(&server_status, &server);
                config.save()?;
            }
            Err(err) => {
                println!("{:?}", err)
            }
        }
        Ok(())
    }
}
