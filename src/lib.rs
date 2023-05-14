extern crate image;
extern crate image_hasher;
use hash_algo::HashAlgorithm;
use napi::bindgen_prelude::*;

mod fetch_buffer;
mod hash_algo;

use fetch_buffer::fetch_buffer;
use image_hasher::{HasherConfig, ImageHash};

#[macro_use]
extern crate napi_derive;

/// get hash of an image
///
/// # Arguments
/// * `input` - Either a Buffer or an image url
/// * `hash_algo` - Optional hash algorithm
///
/// # Example
/// ```js
/// const { getHash } = require('image-distance');
/// const hash = await getHash('https://example.com/image.jpg');
/// ```
#[napi]
pub async fn get_hash(input: Either<Buffer, String>, hash_algo: Option<HashAlgorithm>) {
    let buffer = fetch_buffer(input).await;
    let image = image::load_from_memory(&buffer).unwrap();
    let hasher_config = HasherConfig::new();
    let hasher_config = match hash_algo {
        Some(hash_algo) => hasher_config.hash_alg(hash_algo.into()),
        None => hasher_config,
    };
    let hasher = hasher_config.to_hasher();
    hasher.hash_image(&image).to_base64();
}

/// get hamming distance of two image hashes
///
/// # Arguments
/// * `input1` - Image hash
/// * `input2` - Image hash
///
/// # Example
/// ```js
/// const { getHash, hammingDistanceFromHash } = require('image-distance');
/// const hash1 = await getHash('https://example.com/image1.jpg');
/// const hash2 = await getHash('https://example.com/image2.jpg');
/// const distance = hammingDistanceFromHash(hash1, hash2);
/// ```
#[napi]
pub async fn hamming_distance_from_hash(input1: String, input2: String) {
    let hash1: ImageHash<[u8; 64]> = ImageHash::from_base64(&input1).unwrap();
    let hash2: ImageHash<[u8; 64]> = ImageHash::from_base64(&input2).unwrap();

    hash1.dist(&hash2);
}

/// get hamming distance of two images
///
/// # Arguments
/// * `input1` - Either a Buffer or an image url
/// * `input2` - Either a Buffer or an image url
/// * `hash_algo` - Optional hash algorithm
///
/// # Example
/// ```js
/// const { hammingDistance } = require('image-distance');
/// const distance = await hammingDistance('https://example.com/image1.jpg', 'https://example.com/image2.jpg');
/// ```
#[napi]
pub async fn hamming_distance(
    input1: Either<Buffer, String>,
    input2: Either<Buffer, String>,
    hash_algo: Option<HashAlgorithm>,
) {
    let buffer1 = fetch_buffer(input1).await;
    let buffer2 = fetch_buffer(input2).await;

    let image1 = image::load_from_memory(&buffer1).unwrap();
    let image2 = image::load_from_memory(&buffer2).unwrap();

    let hasher_config = HasherConfig::new();
    let hasher_config = match hash_algo {
        Some(hash_algo) => hasher_config.hash_alg(hash_algo.into()),
        None => hasher_config,
    };

    let hasher = hasher_config.to_hasher();

    let hash1 = hasher.hash_image(&image1);
    let hash2 = hasher.hash_image(&image2);

    hash1.dist(&hash2);
}
