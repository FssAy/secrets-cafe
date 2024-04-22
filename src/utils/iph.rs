use std::net::IpAddr;

pub struct IPHash(String);

impl IPHash {
    pub fn new(ip: IpAddr) -> Self {
        let input = ip.to_string().into_bytes();

        let hash = blake3::hash(&input)
            .to_hex()
            .to_string();

        Self(hash)
    }
}

impl Into<String> for IPHash {
    fn into(self) -> String {
        self.0
    }
}

impl AsRef<str> for IPHash {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
