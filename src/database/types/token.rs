use tokio::io::{AsyncWriteExt, AsyncReadExt, BufReader, BufWriter};

const TOKEN_LIFETIME_S: i64 = 3600;

pub struct SessionToken {
    user_id: String,
    expiration: i64,
}

impl SessionToken {
    pub fn new(user_id: impl ToString) -> Self {
        Self {
            user_id: user_id.to_string(),
            expiration: Self::create_expiration_date(),
        }
    }

    fn create_expiration_date() -> i64 {
        let now = chrono::Utc::now().timestamp();
        now + TOKEN_LIFETIME_S
    }

    /// Encodes itself into an array of bytes.
    ///
    /// # Errors
    /// Should never return an error.
    async fn encode(self) -> tokio::io::Result<Vec<u8>> {
        let buffer_len = std::mem::size_of::<i64>() + self.user_id.len();
        let buffer = Vec::with_capacity(buffer_len);
        let mut writer = BufWriter::with_capacity(buffer_len, buffer);

        writer.write_i64(self.expiration).await?;
        writer.write_all(self.user_id.as_bytes()).await?;

        Ok(writer.into_inner())
    }

    /// Encodes itself into an array of bytes.
    ///
    /// # Errors
    /// Should never return an error IF the buffer is the one returned from the `Self::encode` function.
    async fn decode(buffer: Vec<u8>) -> tokio::io::Result<Self> {
        let mut reader = BufReader::with_capacity(buffer.len(), &*buffer);

        let expiration = reader.read_i64().await?;

        let mut user_id = String::new();
        reader.read_to_string(&mut user_id).await?;

        Ok(Self {
            user_id,
            expiration,
        })
    }
}
