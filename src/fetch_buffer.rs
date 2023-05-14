use napi::{bindgen_prelude::Buffer, Either};

pub async fn fetch_buffer(input: Either<Buffer, String>) -> Vec<u8> {
    match input {
        Either::A(buffer) => buffer.to_vec(),
        Either::B(url) => {
            let res = reqwest::get(&url).await.unwrap();
            res.bytes().await.unwrap().to_vec()
        }
    }
}
