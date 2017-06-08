extern crate swift_keystone;

use swift_keystone::authv2::Client;

fn main() {
    let client = Client::new("http://127.0.0.1:5000/v2.0", "tester", "testing", "test", None).unwrap();
    match client.authenticate() {
        Ok(data) => println!("Token: {} (expired?: {})", data.access.token.id, data.is_token_expired()),
        Err(err) => println!("Error {}", err)
    }
}
