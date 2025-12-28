# Appendix I: Async Rust with Tokio

[Tokio](https://tokio.rs/) is an async runtime for Rust, used in dx for the `chat` and `http` commands.

## Async Basics

### Async Functions

```rust
// Async function
async fn fetch_data() -> String {
    // await other async operations
    let response = make_request().await;
    response
}

// Calling async functions
async fn main_logic() {
    let data = fetch_data().await;
    println!("{}", data);
}
```

### The Runtime

Async code needs a runtime. Tokio provides one:

```rust
use tokio;

#[tokio::main]
async fn main() {
    // Now we can use .await
    let result = async_operation().await;
}
```

Or create runtime manually:

```rust
fn main() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        // Async code here
    });
}
```

## Tokio Setup

### Dependencies

```toml
[dependencies]
tokio = { version = "1", features = ["rt-multi-thread", "macros", "sync"] }
```

### Common Features

| Feature | Provides |
|---------|----------|
| `rt` | Basic runtime |
| `rt-multi-thread` | Multi-threaded runtime |
| `macros` | `#[tokio::main]`, `#[tokio::test]` |
| `sync` | Channels, mutexes |
| `io-std` | Async stdin/stdout |
| `time` | `sleep`, `timeout` |
| `net` | TCP, UDP sockets |
| `fs` | Async file system |

## Spawning Tasks

```rust
use tokio::task;

async fn main() {
    // Spawn a background task
    let handle = task::spawn(async {
        expensive_computation().await
    });

    // Do other work...

    // Wait for result
    let result = handle.await.unwrap();
}
```

### Spawn Blocking

For CPU-intensive or blocking code:

```rust
let result = task::spawn_blocking(|| {
    // This runs on a separate thread pool
    cpu_intensive_work()
}).await.unwrap();
```

## Channels

### mpsc (Multiple Producer, Single Consumer)

```rust
use tokio::sync::mpsc;

async fn main() {
    let (tx, mut rx) = mpsc::channel(32);

    // Sender
    tokio::spawn(async move {
        tx.send("hello").await.unwrap();
    });

    // Receiver
    while let Some(msg) = rx.recv().await {
        println!("Received: {}", msg);
    }
}
```

### broadcast (Multiple Consumers)

```rust
use tokio::sync::broadcast;

let (tx, _rx) = broadcast::channel(16);

// Multiple receivers
let mut rx1 = tx.subscribe();
let mut rx2 = tx.subscribe();

tx.send("hello").unwrap();

// Both rx1 and rx2 receive "hello"
```

## The dx chat Implementation

The `chat` command uses gRPC streaming with Tokio:

```rust
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Streaming};

pub async fn run(args: ChatArgs) -> Result<()> {
    match args.command {
        ChatCommand::Send { room, message } => {
            let mut client = ChatClient::connect(args.server).await?;

            // Create channel for outgoing messages
            let (tx, rx) = mpsc::channel(32);
            let stream = ReceiverStream::new(rx);

            // Send message
            tx.send(ChatMessage { room, content: message }).await?;

            // Start bidirectional stream
            let response = client.chat(Request::new(stream)).await?;
            let mut inbound = response.into_inner();

            // Handle incoming messages
            while let Some(msg) = inbound.message().await? {
                println!("[{}] {}: {}", msg.room, msg.user, msg.content);
            }

            Ok(())
        }
        ChatCommand::Listen { room } => {
            // Subscribe to room messages
            // ...
        }
    }
}
```

## Common Patterns

### Select

Wait for multiple futures:

```rust
use tokio::select;

select! {
    result = async_op1() => {
        println!("Op1 completed: {:?}", result);
    }
    result = async_op2() => {
        println!("Op2 completed: {:?}", result);
    }
}
```

### Timeout

```rust
use tokio::time::{timeout, Duration};

match timeout(Duration::from_secs(5), slow_operation()).await {
    Ok(result) => println!("Completed: {:?}", result),
    Err(_) => println!("Timed out"),
}
```

### Interval

```rust
use tokio::time::{interval, Duration};

let mut interval = interval(Duration::from_secs(1));

loop {
    interval.tick().await;
    println!("Tick!");
}
```

## Async in Sync Context

When you have sync code that needs to call async:

```rust
// In main.rs (sync main)
fn main() -> Result<()> {
    match cli.command {
        // Most commands are sync
        Commands::Hash(args) => commands::hash::run(args),

        // Chat is async - create runtime
        Commands::Chat(args) => {
            tokio::runtime::Runtime::new()
                .expect("Failed to create tokio runtime")
                .block_on(commands::chat::run(args))
        }
    }
}
```

## Testing Async Code

```rust
#[tokio::test]
async fn test_async_function() {
    let result = my_async_function().await;
    assert_eq!(result, expected);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_with_multi_thread() {
    // Uses multi-threaded runtime
}
```

## Resources

- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)
- [Async Book](https://rust-lang.github.io/async-book/)
- [Tokio API Docs](https://docs.rs/tokio)
