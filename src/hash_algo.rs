use image_hasher::HashAlg;

/// Hash algorithms implemented by this crate.
///
/// Implemented primarily based on the high-level descriptions on the blog Hacker Factor
/// written by Dr. Neal Krawetz: http://www.hackerfactor.com/
///
/// Note that `hash_width` and `hash_height` in these docs refer to the parameters of
/// [`HasherConfig::hash_size()`](struct.HasherConfig.html#method.hash_size).
///
/// ### Choosing an Algorithm
/// Each algorithm has different performance characteristics
#[napi]
pub enum HashAlgorithm {
    /// The Mean hashing algorithm.
    ///
    /// The image is converted to grayscale, scaled down to `hash_width x hash_height`,
    /// the mean pixel value is taken, and then the hash bits are generated by comparing
    /// the pixels of the descaled image to the mean.
    ///
    /// This is the most basic hash algorithm supported, resistant only to changes in
    /// resolution, aspect ratio, and overall brightness.
    ///
    /// Further Reading:
    /// http://www.hackerfactor.com/blog/?/archives/432-Looks-Like-It.html
    Mean,

    /// The Median hashing algorithm.
    ///
    /// The image is converted to grayscale, scaled down to `hash_width x hash_height`,
    /// the median pixel value is taken, and then the hash bits are generated by comparing
    /// the pixels of the descaled image to the mean.
    ///
    /// Median hashing in combiantion with preproc_dct is the basis for pHash
    Median,

    /// The Gradient hashing algorithm.
    ///
    /// The image is converted to grayscale, scaled down to `(hash_width + 1) x hash_height`,
    /// and then in row-major order the pixels are compared with each other, setting bits
    /// in the hash for each comparison. The extra pixel is needed to have `hash_width` comparisons
    /// per row.
    ///
    /// This hash algorithm is as fast or faster than Mean (because it only traverses the
    /// hash data once) and is more resistant to changes than Mean.
    ///
    /// Further Reading:
    /// http://www.hackerfactor.com/blog/index.php?/archives/529-Kind-of-Like-That.html
    Gradient,

    /// The Vertical-Gradient hashing algorithm.
    ///
    /// Equivalent to [`Gradient`](#variant.Gradient) but operating on the columns of the image
    /// instead of the rows.
    VertGradient,

    /// The Double-Gradient hashing algorithm.
    ///
    /// An advanced version of [`Gradient`](#variant.Gradient);
    /// resizes the grayscaled image to `(width / 2 + 1) x (height / 2 + 1)` and compares columns
    /// in addition to rows.
    ///
    /// This algorithm is slightly slower than `Gradient` (resizing the image dwarfs
    /// the hash time in most cases) but the extra comparison direction may improve results (though
    /// you might want to consider increasing
    /// [`hash_size`](struct.HasherConfig.html#method.hash_size)
    /// to accommodate the extra comparisons).
    DoubleGradient,

    /// The [Blockhash.io](https://blockhash.io) algorithm.
    ///
    /// Compared to the other algorithms, this does not require any preprocessing steps and so
    /// may be significantly faster at the cost of some resilience.
    ///
    /// The algorithm is described in a high level here:
    /// https://github.com/commonsmachinery/blockhash-rfc/blob/master/main.md
    Blockhash,
}

impl From<HashAlgorithm> for HashAlg {
    fn from(item: HashAlgorithm) -> Self {
        match item {
            HashAlgorithm::Mean => HashAlg::Mean,
            HashAlgorithm::Median => HashAlg::Median,
            HashAlgorithm::Gradient => HashAlg::Gradient,
            HashAlgorithm::VertGradient => HashAlg::VertGradient,
            HashAlgorithm::DoubleGradient => HashAlg::DoubleGradient,
            HashAlgorithm::Blockhash => HashAlg::Blockhash,
        }
    }
}
