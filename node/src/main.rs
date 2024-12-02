//! Node represents an instance of the distributed system in Maelstorm's
//! environment.
use std::io::{self, stdin, stdout, Write};

use messages::MessageType;

type NodeId = String;

#[derive(Default, Debug)]
struct Node {
    id: NodeId,
    other_nodes: Vec<NodeId>,
}

impl Node {
    fn new() -> Self {
        Node::default()
    }

    fn run(&mut self) -> Result<(), io::Error> {
        for stream in stdin().lines() {
            if let Err(e) = stream {
                return Err(e);
            }
            let request = match serde_json::from_str::<messages::Message>(&stream.unwrap()) {
                Ok(r) => r,
                Err(e) => return Err(e.into()),
            };

            if let MessageType::Init { node_id, node_ids } = request.get_type() {
                self.id = node_id;
                self.other_nodes = node_ids;
            }

            let _ = io::stdout().write_fmt(format_args!(
                "{}\n",
                serde_json::to_string(&request.respond())?
            ));

            stdout().flush()?;
        }

        Ok(())
    }
}

fn main() {
    let mut node = Node::new();
    node.run().unwrap();
}
