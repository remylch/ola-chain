mod block;
mod chain;
mod hash;
mod node;
mod peer;

use dotenv::dotenv;
use crate::chain::Chain;
use crate::node::Node;

fn main() {
    dotenv().ok();
    println!("Starting Ola node");
    Chain::load_or_create();
    Node::me().start();
    println!("Stopping Ola node");
}
