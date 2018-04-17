# phoenix-channels-rs

A Rust library providing functionality to connect to a phoenix channel.

It is intended to become at least a production grade client however it is very early in development
and as such should be treated as experimental.

Currently there are two ways to use this library. The first is a high level threaded client
and the second is a low level sender/receiver.


### Connecting via the client

Connecting via the client will return a tuple with the `Client` struct and a `mpsc::Receiver` of
processed incoming messages. This allows for messages to be received in a separate thread if desired.

An very basic example of connecting with the `Client`:

```rust
use std::thread;

extern crate phoenix_channels;
use phoenix_channels::client;


let url = "ws://localhost:4000/socket";

let token = "abcde12345";
let params = vec![("token", token)];

let (client, messages) = client::Client::new(url, params, None).unwrap();

thread::spawn(move || {
    for message in messages {
        println!("{:?}", message);
    }
});

client.join("room:lobby").unwrap();
```

The client itself will handle the heartbeat and anything else required to deliver a serde json struct
to the message iterator result. However, it is an opinionated implementation of a phoenix client and
uses threading, mutexes and channels to achieve its end goal of an easy to use interface. This may not
be desirable for scenarios where more control is required.


### Connecting via the Sender/Receiver

An alternative to the high level `Client` is to purely use the low level `Sender` and `Receiver` structs
and handle the threading and heartbeats yourself. The processing of messages is still handled and methods such
as `join` and `heartbeat` are still conveniently available on the `Sender` struct (the `Client` library
actually uses these under the hood).

Here is an example of how one can use the `Sender`/`Receiver` structs themselves:

```rust
use std::thread;
use std::time::Duration;

extern crate phoenix_channels;
use phoenix_channels::client;

let url = "ws://localhost:4000/socket";

let token = "abcde12345";
let params = vec![("token", token)];

let (mut sender, receiver) = client::connect(url, params, None).unwrap();

sender.join("room:lobby").unwrap();

// create a heartbeat thread
thread::spawn(move || {
    loop {
        sender.heartbeat().unwrap();
        thread::sleep(Duration::from_secs(2));
    }
});

for message in receiver {
    println!("{:?}", message);
}
```

Using the lower level API requires more work but it gives you full control over the threading and flow of
messages being sent and received to the server.


### Logging

Both the high level and low level APIs use slog under the hood for structured logging. To enable this
you can pass your logger via both `connect` and `Client::new` by specifying `Some(logger)` where the
examples currently specify `None`.
