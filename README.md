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