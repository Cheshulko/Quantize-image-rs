# Quantize an image

## Overview
Quantize an image using [k-means](https://en.wikipedia.org/wiki/K-means_clustering) clustering

Inspired by Tsoding

## Usage
```
cargo run -- <path to png image> <K value for K-means>
```
```
cargo run -- assets/images/img_512.png 16
```

Output image will be under `out/<image name>/<K>` folder in the project root folder

## Examples
Images: png, 515x512

| Origin | K=16 | K=8 | K=4 | K=2 |
| -------|---|---|---|---|
| <img src="https://github.com/Cheshulko/Quantize-image-rs/blob/main/assets/images/Lena_512.png?raw=true" width="160" height="160"/> | <img src="https://github.com/Cheshulko/Quantize-image-rs/blob/main/out/Lena_512.png/016.png?raw=true" width="160" height="160"/> | <img src="https://github.com/Cheshulko/Quantize-image-rs/blob/main/out/Lena_512.png/008.png?raw=true" width="160" height="160"/> | <img src="https://github.com/Cheshulko/Quantize-image-rs/blob/main/out/Lena_512.png/004.png?raw=true" width="160" height="160"/> | <img src="https://github.com/Cheshulko/Quantize-image-rs/blob/main/out/Lena_512.png/002.png?raw=true" width="160" height="160"/> |
| <img src="https://github.com/Cheshulko/Quantize-image-rs/blob/main/assets/images/mche_512.png?raw=true" width="160" height="160"/> | <img src="https://github.com/Cheshulko/Quantize-image-rs/blob/main/out/mche_512.png/016.png?raw=true" width="160" height="160"/> | <img src="https://github.com/Cheshulko/Quantize-image-rs/blob/main/out/mche_512.png/008.png?raw=true" width="160" height="160"/> | <img src="https://github.com/Cheshulko/Quantize-image-rs/blob/main/out/mche_512.png/004.png?raw=true" width="160" height="160"/> | <img src="https://github.com/Cheshulko/Quantize-image-rs/blob/main/out/mche_512.png/002.png?raw=true" width="160" height="160"/> |
| <img src="https://github.com/Cheshulko/Quantize-image-rs/blob/main/assets/images/img_512.png?raw=true" width="160" height="160"/> | <img src="https://github.com/Cheshulko/Quantize-image-rs/blob/main/out/img_512.png/016.png?raw=true" width="160" height="160"/> | <img src="https://github.com/Cheshulko/Quantize-image-rs/blob/main/out/img_512.png/008.png?raw=true" width="160" height="160"/> | <img src="https://github.com/Cheshulko/Quantize-image-rs/blob/main/out/img_512.png/004.png?raw=true" width="160" height="160"/> | <img src="https://github.com/Cheshulko/Quantize-image-rs/blob/main/out/img_512.png/002.png?raw=true" width="160" height="160"/> |
| <img src="https://github.com/Cheshulko/Quantize-image-rs/blob/main/assets/images/kyiv_512.png?raw=true" width="160" height="160"/> | <img src="https://github.com/Cheshulko/Quantize-image-rs/blob/main/out/kyiv_512.png/016.png?raw=true" width="160" height="160"/> | <img src="https://github.com/Cheshulko/Quantize-image-rs/blob/main/out/kyiv_512.png/008.png?raw=true" width="160" height="160"/> | <img src="https://github.com/Cheshulko/Quantize-image-rs/blob/main/out/kyiv_512.png/004.png?raw=true" width="160" height="160"/> | <img src="https://github.com/Cheshulko/Quantize-image-rs/blob/main/out/kyiv_512.png/002.png?raw=true" width="160" height="160"/> |

## License
MIT  

