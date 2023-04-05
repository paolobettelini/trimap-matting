# Trimap matting

## CLI Tool

Given a target image and a trimap

```bash
# Generate a soft mask
matting-cli --target target.jpg --trimap trimap.png --save-mask mask.png
# Make the background transparent
matting-cli --target target.jpg --mask mask.png --output out.png --transparent
# Fill the background
matting-cli --target target.jpg --mask mask.png --output out.png --fill "#FFAAB2"
# Replace the background with another image
matting-cli --target target.jpg --mask mask.png --output out.png --replace background.jpg
```

## To install
Install the requirements
```bash
sudo pacman -S libwebp clang qt5-base opencv
```
### Compile the CLI
```bash
cd matting-cli
cargo build --release
```
### Compile the GUI
```bash
cd matting-web
cargo build --release
```

## Compile the documents
```bash
mandate/resources/documentation/compile.sh
mandate/resources/abstract/compile.sh
mandate/resources/diaries/compile.sh
```
The documents will be placed in `mandate/`.