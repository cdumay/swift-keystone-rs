use ::error::Error;
use ::Result;
use hyper::header::{Headers, UserAgent};
use hyper::net::HttpsConnector;
use hyper;
use std::str::FromStr;
use std::time::Duration;
use url::Url;
use hyper_native_tls::NativeTlsClient;

header! { (XAuthUser, "X-Auth-User") => [String] }
header! { (XAuthKey, "X-Auth-Key") => [String] }
header! { (XStorageUrl, "X-Storage-Url") => [String] }
header! { (XAuthToken, "X-Auth-Token") => [String] }

#[derive(Debug)]
pub struct Response {
    pub storage_url: String,
    pub auth_token:String
}

#[derive(Debug)]
pub struct Client {
    headers: Headers,
    timeout: Duration,
    url: Url,
}

impl Client {
    pub fn new(url: &'static str, username: &'static str, password: &'static str, timeout: Option<Duration>) -> Result<Client> {
        let mut headers = Headers::new();

        // User-Agent: CARGO_PKG_NAME/CARGO_PKG_VERSION
        headers.set(UserAgent(format!("{}/{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))));
        // auth headers
        headers.set(XAuthUser(username.to_owned()));
        headers.set(XAuthKey(password.to_owned()));

        Ok(Client {
            headers: headers,
            timeout: timeout.unwrap_or(Duration::new(10, 0)),
            url: Url::from_str(url)?,
        })
    }
    fn headers(&self) -> Headers { self.headers.to_owned() }
    fn timeout(&self) -> Duration { self.timeout }
    fn url(&self) -> Url { self.url.to_owned() }
    pub fn authenticate(&self) -> Result<Response> {
        debug!("Auth V1 <url={}>", self.url());
        let mut client = match self.url.scheme() {
            "https" => {
                let ssl = NativeTlsClient::new()?;
                let connector = HttpsConnector::new(ssl);
                hyper::client::Client::with_connector(connector)
            }
            _ => hyper::client::Client::new(),
        };
        client.set_read_timeout(Some(self.timeout()));
        client.set_redirect_policy(hyper::client::RedirectPolicy::FollowAll);

        let resp = client.get(self.url())
            .headers(self.headers())
            .send()?;

        match resp.status.is_success() {
            true => {
                if (resp.headers.has::<XStorageUrl>() == true) & (resp.headers.has::<XAuthToken>() == true) {
                    debug!("Successfully authenticated on {}", self.url());
                    let ref storage_url = resp.headers.get_raw("X-Storage-Url").unwrap()[0];
                    let ref auth_token = resp.headers.get_raw("X-Auth-Token").unwrap()[0];
                    Ok(Response {
                        storage_url: String::from_utf8(storage_url.to_vec())?,
                        auth_token: String::from_utf8(auth_token.to_vec())?,
                    })
                } else { Err(Error::Generic("Invalid response from server".to_string())) }
            }
            false => {
                let msg = format!("Authentication failed => {}", resp.status.canonical_reason().unwrap());
                error!("{}", &msg);
                Err(Error::Generic(msg))
            }
        }
    }
}