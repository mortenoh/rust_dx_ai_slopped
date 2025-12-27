# chat

Real-time chat using gRPC (tonic).

This command demonstrates gRPC server-streaming with tonic, allowing multiple clients to communicate through a central server.

## Usage

```bash
dx chat server [OPTIONS]     # Start chat server
dx chat client <NAME>        # Connect as client
```

## Subcommands

| Subcommand | Description |
|------------|-------------|
| `server` | Start a chat server |
| `client` | Connect to a server as a chat client |

---

## server

Start a chat server that accepts client connections and broadcasts messages.

```bash
dx chat server [OPTIONS]
```

| Option | Default | Description |
|--------|---------|-------------|
| `-p, --port` | `50051` | Port to listen on |

### Example

```bash
$ dx chat server
✓ Chat server listening on [::1]:50051
Press Ctrl+C to stop

[12:34:56] Alice joined
[12:34:58] Bob joined
[12:35:01] Alice: Hello everyone!
[12:35:03] Bob: Hi Alice!
```

The server:
- Listens for gRPC connections on the specified port
- Broadcasts all messages to connected clients
- Shows join notifications and message activity

---

## client

Connect to a chat server as a client.

```bash
dx chat client <NAME> [OPTIONS]
```

| Argument | Description |
|----------|-------------|
| `NAME` | Your display name in the chat |

| Option | Default | Description |
|--------|---------|-------------|
| `-s, --server` | `http://[::1]:50051` | Server address |

### Example

```bash
$ dx chat client Alice
✓ Connected to http://[::1]:50051
Type messages and press Enter to send. Ctrl+C to quit.

> Hello everyone!
[12:35:03] Bob: Hi Alice!
>
```

The client:
- Connects to the specified server
- Subscribes to receive all broadcast messages
- Sends your typed messages to the server
- Shows messages from other users in real-time

---

## Architecture

The chat system uses gRPC with protocol buffers:

```protobuf
service Chat {
    rpc SendMessage(ChatMessage) returns (Empty);
    rpc Subscribe(SubscribeRequest) returns (stream ChatMessage);
}
```

**Key components:**

1. **Server**: Uses `tokio::sync::broadcast` channel to distribute messages to all subscribers
2. **Client**: Maintains two async tasks - one for receiving messages, one for stdin input
3. **Transport**: tonic gRPC over HTTP/2

---

## Running Multiple Clients

Open multiple terminals:

**Terminal 1 - Server:**
```bash
dx chat server --port 50052
```

**Terminal 2 - Alice:**
```bash
dx chat client Alice --server http://[::1]:50052
```

**Terminal 3 - Bob:**
```bash
dx chat client Bob --server http://[::1]:50052
```

Now Alice and Bob can chat in real-time!

---

## Technical Details

### Dependencies

- `tonic` - gRPC framework
- `prost` - Protocol buffer serialization
- `tokio` - Async runtime with broadcast channels
- `tokio-stream` - Stream utilities for message handling

### Proto Location

The protocol buffer definition is at `proto/chat.proto`.

### Generated Code

The gRPC service code is generated at build time by `tonic-prost-build` and included via:

```rust
pub mod proto {
    tonic::include_proto!("chat");
}
```
