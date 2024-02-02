use base64::Engine;
use hmac::{Hmac, Mac};
use hmac::digest::InvalidLength;
use sha2::Sha256;
use tokio::io::{AsyncWriteExt, AsyncReadExt, BufReader, BufWriter};

const TOKEN_LIFETIME_S: i64 = 3600;
const PACK_TOKEN_SEPARATOR: char = '.';

// todo: Generate random key on compilation
// With public HMAC key anyone can have access to mod accounts!
static HMAC_KEY: &[u8] = include_bytes!("../../secrets/token-hmac-key.bin");

#[derive(Debug)]
pub enum TokenError {
    IO(tokio::io::Error),
    Base64(base64::DecodeError),
    HMAC(InvalidLength),  // Should never happen if using a valid key.
    InvalidSignature,
    ExpiredToken,
    InvalidTokenStructure,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SessionToken {
    pub user_id: String,
    pub expiration: i64,
}

#[derive(Debug, Clone)]
pub struct TokenPack {
    pub token: Vec<u8>,
    pub sign: Vec<u8>,
}

fn make_hmac_sha256() -> Result<Hmac<Sha256>, InvalidLength> {
    Hmac::<Sha256>::new_from_slice(HMAC_KEY)
}

fn now() -> i64 {
    chrono::Utc::now().timestamp()
}

impl SessionToken {
    pub fn new(user_id: impl ToString) -> Self {
        Self {
            user_id: user_id.to_string(),
            expiration: now() + TOKEN_LIFETIME_S,
        }
    }

    /// Encodes itself into an array of bytes.
    ///
    /// # Errors
    /// Should never happen.
    async fn encode(self) -> Result<Vec<u8>, TokenError> {
        let buffer_len = std::mem::size_of::<i64>() + self.user_id.len();
        let buffer = Vec::with_capacity(buffer_len);
        let mut writer = BufWriter::with_capacity(buffer_len, buffer);

        writer.write_i64(self.expiration).await?;
        writer.write_all(self.user_id.as_bytes()).await?;

        Ok(writer.buffer().to_vec())
    }

    /// Encodes itself into an array of bytes.
    ///
    /// # Errors
    /// Should never return an error IF the buffer is the one returned from the `Self::encode` function.
    /// - provided string might not be base64 encoded
    /// - user id might have invalid UTF-8 bytes
    async fn decode(buffer: Vec<u8>) -> Result<Self, TokenError> {
        let mut reader = BufReader::with_capacity(buffer.len(), &*buffer);

        let expiration = reader.read_i64().await?;

        let mut user_id = String::new();
        reader.read_to_string(&mut user_id).await?;

        Ok(Self {
            user_id,
            expiration,
        })
    }

    /// Encodes the token and creates a signature for it.
    ///
    /// # Errors
    /// Should never happen.
    pub async fn sign(self) -> Result<TokenPack, TokenError> {
        let token = self.encode().await?;

        let mut mac = make_hmac_sha256()?;
        mac.update(&token);
        let sign = mac.finalize().into_bytes().to_vec();

        Ok(TokenPack {
            token,
            sign,
        })
    }

    /// Decodes the token, verifies its signature and expiration date.
    ///
    /// # Errors
    /// Can fail even if the token and signature is valid when the expiration date is reached.
    /// - same as for `Self::decode`
    /// - token signature is invalid
    /// - token is expired
    pub async fn verify(pack: TokenPack) -> Result<Self, TokenError> {
        let mut mac = make_hmac_sha256()?;
        mac.update(&pack.token);

        // Without signature verification anyone can have access to mod accounts!
        mac.verify_slice(&*pack.sign).map_err(|_| TokenError::InvalidSignature)?;

        let token = Self::decode(pack.token).await?;
        if now() > token.expiration {
            return Err(TokenError::ExpiredToken);
        }

        Ok(token)
    }
}

impl TokenPack {
    pub fn pack(self) -> String {
        let b64 = base64::engine::general_purpose::STANDARD;

        let token = b64.encode(self.token);
        let sign = b64.encode(self.sign);

        format!("{}{}{}", token, PACK_TOKEN_SEPARATOR, sign)
    }

    pub fn unpack(packed: String) -> Result<Self, TokenError> {
        let (token, sign) = packed
            .split_once(PACK_TOKEN_SEPARATOR)
            .ok_or_else(|| TokenError::InvalidTokenStructure)?;

        Ok(Self {
            token: token.to_string().into_bytes(),
            sign: sign.to_string().into_bytes(),
        })
    }
}

impl From<tokio::io::Error> for TokenError {
    fn from(err: tokio::io::Error) -> Self {
        Self::IO(err)
    }
}

impl From<base64::DecodeError> for TokenError {
    fn from(err: base64::DecodeError) -> Self {
        Self::Base64(err)
    }
}

impl From<InvalidLength> for TokenError {
    fn from(err: InvalidLength) -> Self {
        Self::HMAC(err)
    }
}
