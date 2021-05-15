# Imgopt

just a binary that optimzes images

## Getting started

`git clone` and then:

```sh
cargo build --release
```

Then your binary can be found in target/release folder.

```sh
# prints the help
imgopt --help

# usage
imgopt [input] [output]

# examples
imgopt input.png output.png
imgopt input.jpeg output.jpeg
imgopt input.svg output.svg
imgopt input.png output.webp
imgopt input.jpeg output.webp
```

## formats

- jpeg: [mozjpeg](https://github.com/ImageOptim/mozjpeg-rust)
- png: pngquant ([libimagequant](https://github.com/ImageOptim/libimagequant))
- svg: [svgcleaner](https://github.com/RazrFalcon/svgcleaner)
- web: [webp](https://github.com/jaredforth/webp) (currently only outputs to webp, with quality set to 80)

## test images

jpg images from <https://www.pexels.com>

png images from <https://www.cleanpng.com>

svg images from <https://freesvg.org>
