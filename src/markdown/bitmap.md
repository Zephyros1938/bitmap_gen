# Bitmap File Structure

The **bitmap file structure** is commonly associated with image files, especially in formats like BMP (Bitmap Image File), which store images as a grid of pixels. A bitmap image is made up of individual pixels (picture elements), and each pixel can represent a specific color. The structure of a bitmap file defines how this pixel data, along with other associated information, is organized within the file.

A typical bitmap file (such as BMP) has the following main components:

## 1. File Header (14 bytes)
This header provides general information about the bitmap file and is used to identify the file type.

- **Signature (2 bytes)**: Always `0x42 0x4D` (ASCII characters "BM"), indicating it's a BMP file.
- **File Size (4 bytes)**: The total size of the file, including the file header, info header, and image data.
- **Reserved Fields (4 bytes)**: These are typically unused fields, set to zero.
- **Data Offset (4 bytes)**: The offset (in bytes) from the beginning of the file to the start of the image data (this is where the pixel data begins).

## 2. DIB Header (Bitmap Information Header, typically 40 bytes)
This header contains detailed information about the image itself, such as dimensions, color depth, and compression. It provides more specific details about how the pixel data should be interpreted.

- **Header Size (4 bytes)**: The size of this header (40 bytes for standard BMP files).
- **Image Width (4 bytes)**: The width of the image in pixels.
- **Image Height (4 bytes)**: The height of the image in pixels.
- **Color Planes (2 bytes)**: Always set to 1 (indicating a single color plane).
- **Bits per Pixel (2 bytes)**: The number of bits used for each pixel. Common values are 1 (monochrome), 4, 8, 16, 24, or 32 (higher values provide more color depth).
- **Compression Type (4 bytes)**: Specifies the type of compression used (0 for no compression, 1 for RLE compression, etc.).
- **Image Size (4 bytes)**: The size of the image data (not the entire file).
- **Horizontal Resolution (4 bytes)**: The horizontal resolution of the image in pixels per meter.
- **Vertical Resolution (4 bytes)**: The vertical resolution of the image in pixels per meter.
- **Colors in Palette (4 bytes)**: The number of colors used in the image's color palette (only applicable for images with a limited color palette, e.g., 8-bit).
- **Important Colors (4 bytes)**: The number of important colors, typically 0 for full-color images, or fewer if the image has a limited palette.

## 3. Color Palette (optional)
For images with a limited color depth (e.g., 1-bit, 4-bit, 8-bit), a color palette is used to map pixel values to actual colors. This is essentially a table of RGB (Red, Green, Blue) values, with each entry describing a color.

- **RGB Values**: Each entry typically uses 4 bytes (1 byte each for blue, green, red, and 1 unused byte) to store the color. The size of the palette is determined by the number of colors allowed by the image (e.g., 256 colors for an 8-bit image).

## 4. Pixel Data
This is the core of the bitmap file and contains the actual image data.

- **Pixel Format**: The format of the pixel data depends on the bits per pixel specified in the DIB header. For example:
  - **1-bit**: Each pixel is either black or white.
  - **8-bit**: Each pixel is an index into the color palette.
  - **24-bit**: Each pixel uses 3 bytes (one for each color channel: red, green, and blue).
  - **32-bit**: Each pixel uses 4 bytes, with an extra byte for transparency (alpha channel).
  
- **Data Layout**: The pixel data is arranged from the bottom row to the top row (inverted compared to other formats like PNG). Each row is padded to ensure that its size is a multiple of 4 bytes, which can result in extra padding at the end of each row.

## 5. File Footer
In some BMP files, there may be a footer or additional metadata, though this is less common in basic BMP files.

---

### Summary of the Bitmap File Structure:

1. **File Header (14 bytes)** – Information about the file itself.
2. **DIB Header (40 bytes)** – Information about the image (width, height, color depth).
3. **Color Palette (optional)** – For images with a limited color palette (e.g., 8-bit images).
4. **Pixel Data** – Actual image content, stored in rows of pixels.

In simple terms, the bitmap file structure begins with a header that defines the file's size and type, followed by another header containing details about the image itself (size, color depth, etc.), potentially a color palette (for images with fewer colors), and finally the pixel data that forms the image.

This structure ensures that the bitmap format is flexible for various image types while keeping it relatively simple to decode and render.
