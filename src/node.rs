use crate::peer::PeerNode;
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use std::net::{IpAddr, TcpListener, TcpStream};
use std::{env, io, thread};
use std::sync::{Arc, Mutex};

pub(crate) trait NodeInfo {
    fn ip(&self) -> IpAddr;
    fn port(&self) -> u16;
    fn socket_addr(&self) -> String {
        format!("{}:{}", self.ip(), self.port())
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct Node {
    ip: IpAddr,
    port: u16,
    #[serde(skip)]
    peers: Vec<PeerNode>,
}

impl NodeInfo for Node {
    fn ip(&self) -> IpAddr {
        self.ip
    }

    fn port(&self) -> u16 {
        self.port
    }
}

impl Node {

    pub(crate) fn me() -> Self {
        match (env::var("NODE_IP"), env::var("NODE_PORT")) {
            (Ok(ip_str), Ok(port_str)) => {
                match (ip_str.trim().parse::<IpAddr>(), port_str.trim().parse::<u16>()) {
                    (Ok(ip), Ok(port)) => {
                        let peers = PeerNode::get_peers_node_ips_from_env();
                        Node { ip, port, peers }
                    },
                    (Err(_), _) => panic!("Failed to parse NODE_IP as IpAddr"),
                    (_, Err(_)) => panic!("Failed to parse NODE_PORT as u16"),
                }
            }
            (Err(_), _) => panic!("NODE_IP environment variable is not set"),
            (_, Err(_)) => panic!("NODE_PORT environment variable is not set"),
        }
    }

    pub(crate) fn start(self) {
        let node = Arc::new(Mutex::new(self));
        
        let node_listener = Arc::clone(&node);
        let listener_thread = thread::spawn(move || {
            let mut node = node_listener.lock().unwrap();
            node.listen_for_connections();
        });
        
        let node_peers = Arc::clone(&node);
        let peer_thread = thread::spawn(move || {
            let node = node_peers.lock().unwrap();
            node.contact_peers();
        });
        
        listener_thread.join().unwrap();
        peer_thread.join().unwrap();
    }

    pub(crate) fn connect(&self) -> io::Result<TcpStream> {
        let socket = (self.ip, 8080);
        TcpStream::connect(socket)
    }

    fn handle_client(&mut self, mut stream: TcpStream) {
        let mut buffer = [0; 1024];

        loop {
            match stream.read(&mut buffer) {
                Ok(n) if n == 0 => {
                    println!("Connection closed by client");
                    return;
                }
                Ok(n) => {
                    let message = String::from_utf8_lossy(&buffer[..n]);
                    println!("Received message: {}", message);

                    if message.starts_with("SYNC_REQUEST") {
                        if let Ok(peer_addr) = stream.peer_addr() {
                            let peer_node = PeerNode::new(peer_addr.ip(), peer_addr.port());
                            self.peers.push(peer_node);
                            println!("New peer registered: {}", peer_addr);
                        }

                        let response = "SYNC_RESPONSE".as_bytes();
                        if let Err(e) = stream.write_all(response) {
                            eprintln!("Failed to send sync response: {}", e);
                            return;
                        }
                    } else {
                        // Echo other messages
                        if let Err(e) = stream.write(&buffer[..n]) {
                            eprintln!("Failed to send response: {}", e);
                            return;
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Failed to read from connection: {}", e);
                    return;
                }
            }
        }
    }

    fn listen_for_connections(&mut self) {
        let listener = TcpListener::bind(self.socket_addr()).expect("Failed to bind to address");
        println!("Node is now listening on {}", self.socket_addr());

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    println!("New connection from : {}", stream.peer_addr().unwrap());
                    self.handle_client(stream);
                }
                Err(e) => {
                    eprintln!("Connection failed : {}", e);
                }
            }
        }
    }

    fn contact_peers(&self) {
        self.peers.iter().for_each(|peer| {
            match (self.connect_to_peer(peer)) {
                Ok(mut stream) => {
                    println!("Syncing with peer: {}...", peer.socket_addr());

                    // Send sync request
                    let sync_message = "SYNC_REQUEST".as_bytes();
                    if let Err(e) = stream.write_all(sync_message) {
                        eprintln!("Failed to send sync request to {}: {}", peer.socket_addr(), e);
                        return;
                    }

                    eprintln!("Synced with peer: {}", peer.socket_addr());
                }
                Err(e) => {
                    eprintln!("Failed to sync with peer {}: {}", peer.socket_addr(), e);
                }
            }
        })
    }

    fn connect_to_peer(&self, peer: &PeerNode) -> io::Result<TcpStream> {
        let socket = (peer.ip(), peer.port());
        TcpStream::connect(socket)
    }

}
