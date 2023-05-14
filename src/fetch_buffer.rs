use napi::{bindgen_prelude::Buffer, Either, Error, Status};

pub async fn fetch_buffer(input: Either<Buffer, String>) -> Result<Vec<u8>, Error> {
    match input {
        Either::A(buffer) => Ok(buffer.to_vec()),
        Either::B(url) => {
            let res = reqwest::get(&url).await.map_err(|e| {
                Error::new(
                    Status::InvalidArg,
                    format!("Image fetching failed with error: {}", e.to_string()),
                )
            })?;

            let bytes = res
                .bytes()
                .await
                .map_err(|e| {
                    Error::new(
                        Status::InvalidArg,
                        format!(
                            "Getting bytes from url failed with error: {}",
                            e.to_string()
                        ),
                    )
                })?
                .to_vec();

            Ok(bytes)
        }
    }
}
