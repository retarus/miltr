# Miltr Client

A client implementation of the milter protocol.

```rust no_run
use tokio::net::TcpStream;
use tokio_util::compat::TokioAsyncReadCompatExt;

use miltr_client::Client;
use miltr_common::{
    commands::Connect,
    optneg::OptNeg,
};

#[tokio::main]
async fn main() {
    let mut stream = TcpStream::connect("localhost:8080")
        .await
        .expect("Failed connecting to milter server")
        .compat();

    println!("Opened TCP connection");
    let options = OptNeg::default();
    let client = Client::new(options);
    let mut connection = client
        .connect_via(&mut stream)
        .await
        .expect("Failed to setup connection");

    // Further processing
}
```

Currently, the milter client is relatively barebone. It should handle all
commands, actions and modification actions correctly. But, everything else is
basically up to the user. For example, nothing prevents you from claiming one
behavior in option negotiation, but actually doing something else.

The use case for this client library currently is to have an example client to
mess around and test behavior with.
