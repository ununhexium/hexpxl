# Hexagonal Pixel

Pixelises using a hexagonal pattern instead of a classic square pattern.

Supported file formats: [rust image crate](https://docs.rs/image/0.21.2/image/)
Processing time: ~0.3s on 4k screen with single threaded AMD Ryzen 5 2600X

## Build

`cargo build --release`

## Install

`cargo install`

## Usage

`hexpxl --help`

### Example

![input with normal pixels](https://raw.githubusercontent.com/ununhexium/hexpxl/master/doc/input.png)

`hexpxl input.png output.png 10 --mode hex`

![output with large pixels](https://raw.githubusercontent.com/ununhexium/hexpxl/master/doc/output.png)


