# swift-keystone-rs
RUST keystone client

```toml
# Cargo.toml
[dependencies]
swift-keystone = { git = "https://github.com/cdumay/swift-keystone-rs.git" }
```

### Auth v1

```rust
extern crate swift_keystone;

use swift_keystone::authv1::Client;

fn main() {
    let client = Client::new("http://127.0.0.1:5000/v1.0", "tester", "testing", None).unwrap();
    match client.authenticate() {
        Ok(data) => println!("Storage URL: {}, Token: {}", data.storage_url, data.auth_token),
        Err(err) => println!("Error {}", err)
    }
}
```

### Auth v2

```rust
extern crate swift_keystone;

use swift_keystone::authv2::Client;

fn main() {
    let client = Client::new("http://127.0.0.1:5000/v2.0", "tester", "testing", "test", None).unwrap();
    match client.authenticate() {
        Ok(data) => println!("Token: {} (expired?: {})", data.access.token.id, data.is_token_expired()),
        Err(err) => println!("Error: {}", err)
    }
}
```

## License
Licensed under MIT license ([LICENSE-MIT](LICENSE) or http://opensource.org/licenses/MIT)