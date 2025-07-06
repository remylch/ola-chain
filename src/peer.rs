use serde::{Deserialize, Serialize};
use std::env;
use std::net::IpAddr;
use crate::node::NodeInfo;

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct PeerNode {
    ip: IpAddr,
    port: u16,
}

impl NodeInfo for PeerNode {
    fn ip(&self) -> IpAddr {
        self.ip
    }

    fn port(&self) -> u16 {
        self.port
    }

    fn socket_addr(&self) -> String {
        format!("{}:{}", self.ip(), self.port())
    }
}

impl PeerNode {
    pub(crate) fn new(ip: IpAddr, port: u16) -> Self {
        PeerNode { ip, port }
    }

    pub(crate) fn get_peers_node_ips_from_env() -> Vec<PeerNode> {
        match env::var("NODES") {
            Ok(ips) => ips
                .split(',')
                .filter_map(|socket_addr| {
                    let parts: Vec<&str> = socket_addr.trim().split(':').collect();
                    if parts.len() == 2 {
                        match (parts[0].parse::<IpAddr>(), parts[1].parse::<u16>()) {
                            (Ok(ip), Ok(port)) => Some(PeerNode::new(ip, port)),
                            _ => {
                                eprintln!(
                                    "Invalid socket address in NODES environment variable: {}",
                                    socket_addr
                                );
                                None
                            }
                        }
                    } else {
                        eprintln!(
                            "Invalid format in NODES environment variable: {}. Expected IP:PORT",
                            socket_addr
                        );
                        None
                    }
                })
                .collect::<Vec<PeerNode>>(),
            Err(_) => {
                println!("No NODES peer provided");
                Vec::new()
            }
        }
    }

}