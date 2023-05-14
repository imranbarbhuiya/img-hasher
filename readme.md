# Image Hasher

A fast image hash generator and hamming distance calculator using multiple algorithms

## Installation

```bash
npm install img-hasher
```

## Usage

### Get Hash Of an Image

```js
const { getHash } = require('img-hasher');
const hash = await getHash('https://example.com/image.jpg');
```

### Get Hash Distance Between Two Images

```js
const { hammingDistance } = require('img-hasher');
const distance = await hammingDistance('https://example.com/image1.jpg', 'https://example.com/image2.jpg');
```

### Get Hash Distance Between Two Hashes

```js
const { getHash, hammingDistanceFromHash } = require('img-hasher');
const hash1 = await getHash('https://example.com/image1.jpg');
const hash2 = await getHash('https://example.com/image2.jpg');
const distance = hammingDistanceFromHash(hash1, hash2);
```

### Algorithms

- Mean
- Gradient
- VertGradient
- DoubleGradient
- BlockHash
