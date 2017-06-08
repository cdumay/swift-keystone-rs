use ::error::Error;
use ::Result;
use chrono::datetime::DateTime;
use chrono::UTC;
use hyper::header::{Headers, ContentType, Accept, UserAgent};
use hyper::net::HttpsConnector;
use hyper;
use serde_json;
use std::io::Read;
use std::str::FromStr;
use std::time::Duration;
use url::Url;
use hyper_native_tls::NativeTlsClient;

#[derive(Debug, Serialize)]
struct PayloadAuthCredentials {
    username: &'static str,
    password: &'static str
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct PayloadAuth {
    password_credentials: PayloadAuthCredentials,
    tenant_name: &'static str
}

#[derive(Debug, Serialize)]
struct Payload {
    auth: PayloadAuth,
}

#[derive(Debug, Deserialize)]
pub struct ResponseAccessIdName {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub name: String,
}


#[derive(Debug, Deserialize)]
pub struct ResponseAccessToken {
    pub id: String,
    pub expires: DateTime<UTC>,
    pub tenant: ResponseAccessIdName,
}

#[derive(Debug, Deserialize)]
pub struct ResponseAccessUser {
    pub id: String,
    pub name: String,
    pub roles: Vec<ResponseAccessIdName>
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseAccessServiceCatalogEndpoint {
    pub region: String,
    #[serde(rename = "publicURL")]
    pub public_url: String,
    pub id: String
}

#[derive(Debug, Deserialize)]
pub struct ResponseAccessServiceCatalog {
    pub endpoints: Vec<ResponseAccessServiceCatalogEndpoint>,
    pub name: String,
    #[serde(rename = "type")]
    pub _type: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseAccess {
    pub token: ResponseAccessToken,
    pub user: ResponseAccessUser,
    pub service_catalog: Vec<ResponseAccessServiceCatalog>
}

#[derive(Debug, Deserialize)]
pub struct Response {
    pub access: ResponseAccess,
}

impl Response {
    pub fn is_token_expired(&self) -> bool {
        self.access.token.expires < UTC::now()
    }
}

#[derive(Debug, Deserialize)]
pub struct ErrorResponseError {
    message: String,
    code: i32,
    title: String
}

#[derive(Debug, Deserialize)]
pub struct ErrorResponse {
    error: ErrorResponseError,
}

#[derive(Debug)]
pub struct Client {
    headers: Headers,
    password: &'static str,
    tenant: &'static str,
    timeout: Duration,
    url: Url,
    username: &'static str,
}

impl Client {
    pub fn new(url: &'static str, username: &'static str, password: &'static str, tenant: &'static str, timeout: Option<Duration>) -> Result<Client> {
        let mut headers = Headers::new();

        // Content-Type: application/json
        headers.set(ContentType::json());
        // Accept: application/json
        headers.set(Accept::json());
        // User-Agent: CARGO_PKG_NAME/CARGO_PKG_VERSION
        headers.set(UserAgent(format!("{}/{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))));

        Ok(Client {
            headers: headers,
            password: password,
            tenant: tenant,
            timeout: timeout.unwrap_or(Duration::new(10, 0)),
            url: Url::from_str(url)?.join("/tokens")?,
            username: username,
        })
    }
    fn headers(&self) -> Headers { self.headers.to_owned() }
    fn password(&self) -> &'static str { self.password }
    fn tenant(&self) -> &'static str { self.tenant }
    fn timeout(&self) -> Duration { self.timeout }
    fn url(&self) -> Url { self.url.to_owned() }
    fn username(&self) -> &'static str { self.username }
    fn payload(&self) -> Payload {
        Payload {
            auth: PayloadAuth {
                password_credentials: PayloadAuthCredentials {
                    username: self.username(),
                    password: self.password(),
                },
                tenant_name: self.tenant()
            }
        }
    }

    pub fn authenticate(&self) -> Result<Response> {
        debug!("Auth V2 <url={}, username={}, tenant={}>", self.url(), self.username(), self.tenant());
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

        let payload = self.payload();
        let payload = serde_json::to_string(&payload).unwrap();

        let mut resp = client.post(self.url())
            .headers(self.headers())
            .body(payload.as_str())
            .send()?;

        let mut jdata = String::new();
        resp.read_to_string(&mut jdata)?;

        match resp.status.is_success() {
            true => {
                debug!("Successfully authenticated on {}", self.url());
                let data: Response = serde_json::from_str(&jdata)?;
                Ok(data)
            }
            false => {
                error!("Authentication failed => {}", resp.status.canonical_reason().unwrap());
                let data: ErrorResponse = serde_json::from_str(&jdata)?;
                Err(Error::from(format!("{} {}: {}", data.error.code, data.error.title, data.error.message)))
            }
        }
    }
}
