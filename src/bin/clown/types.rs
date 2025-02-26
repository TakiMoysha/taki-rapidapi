use serde::Deserialize;

pub const BIND_ADDR: &str = "0.0.0.0:8080";

fn default_ip_addr() -> String {
    String::from(BIND_ADDR)
}

#[derive(Deserialize, Debug)]
pub struct StreamingContext {
    #[serde(default = "default_ip_addr")]
    pub ip_addr: String,
}

impl StreamingContext {
    pub fn from_json(input: &str) -> Self {
        Self {
            ..Default::default()
        }
    }
}

impl Default for StreamingContext {
    fn default() -> Self {
        Self {
            ip_addr: String::from(BIND_ADDR),
        }
    }
}
