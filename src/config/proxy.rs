#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Proxy {
    #[serde(default)]
    pub http: String,
    #[serde(default)]
    pub https: String,
    #[serde(default)]
    pub all: String,
}

impl super::Config {
    #[allow(dead_code)]
    pub fn get_http_proxy(&self) -> &String {
        &self.proxy.http
    }

    #[allow(dead_code)]
    pub fn get_https_proxy(&self) -> &String {
        &self.proxy.https
    }

    #[allow(dead_code)]
    pub fn get_all_proxy(&self) -> &String {
        &self.proxy.all
    }
}
