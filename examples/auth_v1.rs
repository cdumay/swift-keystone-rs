extern crate swift_keystone;

use swift_keystone::authv1::Client;

fn main() {
    let client = Client::new("http://127.0.0.1:5000/v1.0", "tester", "testing", None).unwrap();
    match client.authenticate() {
        Ok(data) => println!("Storage URL: {}, Token: {}", data.storage_url, data.auth_token),
        Err(err) => println!("Error {}", err)
    }
}
