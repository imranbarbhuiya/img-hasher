# Image Distance

A fast and easy to use image hash distance calculator using multiple algorithms.

## Installation

```bash
npm install image-distance
```

## Usage

### Get Hash Of an Image

```js
const { getHash } = require('image-distance');
const hash = await getHash('https://example.com/image.jpg');
```

### Get Hash Distance Between Two Images

```js
const { hammingDistance } = require('image-distance');
const distance = await hammingDistance('https://example.com/image1.jpg', 'https://example.com/image2.jpg');
```

### Get Hash Distance Between Two Hashes

```js
const { getHash, hammingDistanceFromHash } = require('image-distance');
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
