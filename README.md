# maelstorm-challenge
Fly-io's distributed systems challenge solution in Rust.

GitHub Link: https://github.com/jepsen-io/maelstrom

### Maelstorm Challenge Environment

- Node('s): An instance of a distributed service that recieves data, sends
acknowledgements & is reponsible for syncronizing state accross the other
Node's. A Node is what we implement in the challenge.
- Client: Maelstorm executable that sets up the networking layer and emulates
requests to Node's. 