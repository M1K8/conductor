use reqwest::Client;
use std::{net::IpAddr, time::Duration};

use super::Device;

pub struct Moonraker {
    client: reqwest::Client,
}

impl Default for Moonraker {
    fn default() -> Self {
        Self {
            client: Default::default(),
        }
    }
}

impl Moonraker {
    pub async fn test_ping(self, srv: IpAddr) -> Result<(), Box<dyn std::error::Error>> {
        let mut svr_str = "http://".to_owned();
        svr_str.push_str(srv.to_string().as_str());
        let resp = self.client.get(svr_str).send().await?;
        resp.error_for_status()?;

        Ok(())
    }

    pub fn new() -> Self {
        let cl = Client::builder()
            .timeout(Duration::from_millis(100))
            .build();

        match cl {
            Ok(c) => Moonraker { client: c },
            Err(e) => panic!("{:?}", e),
        }
    }
}

impl Device for Moonraker {
    fn print(&self, f: &super::PrintFile) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    fn upload(&self, f: &super::PrintFile) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    fn get_info(&self) -> std::collections::HashMap<String, String> {
        todo!()
    }
}
