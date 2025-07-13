mod block;
mod chain;
mod hash;
mod node;
mod peer;
mod store;
mod transaction;
mod address;
mod block_builder;
mod transaction_pool;

use dotenv::dotenv;
use crate::chain::Chain;
use crate::node::Node;

fn main() {
    dotenv().ok();
    println!("Starting Ola node");
    let chain = Chain::load_or_create();
    Node::me(chain).start();
    println!("Stopping Ola node");
}
