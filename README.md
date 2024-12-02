# maelstorm-challenge
Fly-io's distributed systems challenge solution in Rust.

GitHub Link: https://github.com/jepsen-io/maelstrom

### Maelstorm Challenge Environment

- Node('s): An instance of a distributed service that recieves data, sends
acknowledgements & is reponsible for syncronizing state accross the other
Node's. A Node is what we implement in the challenge.
- Client: Maelstorm executable that sets up the networking layer and emulates
requests to Node's. 

### Challenge 0: Init

- This challenge is not really outlined as a test in the Maelstorm challenge.
However, the `init` & `init_ok` are the first requests and reponses that a
Node would recieve in the Maelstorm environment.

- The `init` request seems to outline all the nodes Maelstorms distributed
system. The node is given a unique ID & a list of ID's associated with other
nodes in the environment.

##### Example local test:

> [!TIP]
> All commands shown are run from the GitHub repo's root
> i.e. maelstorm-challenge.

1. Run the node:

```
cargo run node
```

2. Paste the `init` json string to `STDIN`:

```
{"src": "c1", "dest": "n1", "body": {"type": "init", "msg_id": 1, "node_id": "n3", "node_ids": ["n1", "n2", "n3"]}}
```

3. You should see `init_ok` response from node to `STDOUT`:

```
{"src":"n1","dest":"c1","body":{"type":"init_ok","in_reply_to":1}}
```

### Challenge 1: Echo

- In this challenge, we implement `echo` & `echo_ok` RPC calls.

##### Example local test:

> [!TIP]
> Our node implementation loops over the `STDIN` stream to read responses until
> it see's a `EOF` token. So a graceful way to kill the node is to use
> `Ctrl + d` which sends a `EOF` token.
> 
> Using `Ctrl + c` does **not** exit the node gracefully.

1. Run the node:

```
cargo run node
```

2. Paste the `echo` json string to `STDIN`:

```
{"src": "c1", "dest": "n1", "body": {"msg_id": 1, "type": "echo", "echo": "hello there"}}
```

3. You should see `echo_ok` response from node to `STDOUT`:

```
{"src":"n1","dest":"c1","body":{"type":"echo_ok","echo":"hello there","in_reply_to":1}}
```

##### Run the challenge:

1. Build the node executable:

```
cargo build .
```

2. Execute the `maelstorm` client against the node:

```
./client/maelstrom test -w echo --bin ./target/debug/node --node-count 1 --time-limit 10
```

3. Check the results. If everyything works, you should see in `STDOUT`:

```
Everything looks good! ヽ(‘ー`)ノ
```