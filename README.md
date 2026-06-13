# Image Quantizer

A Rust implementation of image color quantization using the **K-means clustering** algorithm.

## Overview
This tool reduces the color palette of an image by grouping similar pixels together. By treating each pixel's RGB values as coordinates in 3D space, the K-means algorithm identifies the $K$ most representative colors (centroids) and maps every pixel in the original image to its nearest centroid.

**Inspired by:** [Tsoding Daily](https://www.youtube.com/@tsodingdaily)

## How it Works
1.  **Pixel Mapping:** The program reads a PNG image and treats each pixel as a data point in 3D (Red, Green, Blue) space.
2.  **Clustering:** It runs the K-means algorithm to find $K$ clusters of colors.
3.  **Quantization:** Every pixel is replaced with the color of its nearest cluster center.
4.  **Output:** The result is an image that looks similar to the original but contains only $K$ unique colors.

## Usage
Ensure you have the [Rust toolchain](https://rustup.rs/) installed.

```bash
cargo run -- <path_to_png> <K_value>
```

### Example:
```bash
cargo run -- assets/images/img_512.png 16
```

### Output
The processed image will be saved in the project root using the following directory structure:
`out/<image_name>/<K>.png`

For example, if you run a K-value of 16 on `Lena.png`, the result will be at:
`out/Lena.png/016.png`

## Examples
The following examples were processed using **512x512** PNG images. Notice how the visual fidelity changes as the color palette ($K$) is reduced.

| Original | K=16 | K=8 | K=4 | K=2 |
| :--- | :--- | :--- | :--- | :--- |
| <img src="https://github.com/Cheshulko/Quantize-image-rs/blob/main/assets/images/Lena_512.png?raw=true" width="160"/> | <img src="https://github.com/Cheshulko/Quantize-image-rs/blob/main/out/Lena_512.png/016.png?raw=true" width="160"/> | <img src="https://github.com/Cheshulko/Quantize-image-rs/blob/main/out/Lena_512.png/008.png?raw=true" width="160"/> | <img src="https://github.com/Cheshulko/Quantize-image-rs/blob/main/out/Lena_512.png/004.png?raw=true" width="160"/> | <img src="https://github.com/Cheshulko/Quantize-image-rs/blob/main/out/Lena_512.png/002.png?raw=true" width="160"/> |
| <img src="https://github.com/Cheshulko/Quantize-image-rs/blob/main/assets/images/mche_512.png?raw=true" width="160"/> | <img src="https://github.com/Cheshulko/Quantize-image-rs/blob/main/out/mche_512.png/016.png?raw=true" width="160"/> | <img src="https://github.com/Cheshulko/Quantize-image-rs/blob/main/out/mche_512.png/008.png?raw=true" width="160"/> | <img src="https://github.com/Cheshulko/Quantize-image-rs/blob/main/out/mche_512.png/004.png?raw=true" width="160"/> | <img src="https://github.com/Cheshulko/Quantize-image-rs/blob/main/out/mche_512.png/002.png?raw=true" width="160"/> |
| <img src="https://github.com/Cheshulko/Quantize-image-rs/blob/main/assets/images/img_512.png?raw=true" width="160"/> | <img src="https://github.com/Cheshulko/Quantize-image-rs/blob/main/out/img_512.png/016.png?raw=true" width="160"/> | <img src="https://github.com/Cheshulko/Quantize-image-rs/blob/main/out/img_512.png/008.png?raw=true" width="160"/> | <img src="https://github.com/Cheshulko/Quantize-image-rs/blob/main/out/img_512.png/004.png?raw=true" width="160"/> | <img src="https://github.com/Cheshulko/Quantize-image-rs/blob/main/out/img_512.png/002.png?raw=true" width="160"/> |
| <img src="https://github.com/Cheshulko/Quantize-image-rs/blob/main/assets/images/kyiv_512.png?raw=true" width="160"/> | <img src="https://github.com/Cheshulko/Quantize-image-rs/blob/main/out/kyiv_512.png/016.png?raw=true" width="160"/> | <img src="https://github.com/Cheshulko/Quantize-image-rs/blob/main/out/kyiv_512.png/008.png?raw=true" width="160"/> | <img src="https://github.com/Cheshulko/Quantize-image-rs/blob/main/out/kyiv_512.png/004.png?raw=true" width="160"/> | <img src="https://github.com/Cheshulko/Quantize-image-rs/blob/main/out/kyiv_512.png/002.png?raw=true" width="160"/> |

## License
This project is licensed under the MIT License.
