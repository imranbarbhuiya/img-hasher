use napi::{bindgen_prelude::Buffer, Either};

pub async fn fetch_buffer(input: Either<Buffer, String>) -> Vec<u8> {
    match input {
        Either::A(buffer) => buffer.to_vec(),
        Either::B(url) => {
            let res = reqwest::get(&url).await.expect("Failed to fetch image");
            res.bytes()
                .await
                .expect("Can't fetch buffer from the provided url")
                .to_vec()
        }
    }
}
