#[repr(C)] // Ensures the struct layout matches C-style layout
#[derive(Debug, Clone, Copy)]
pub struct BitmapFileHeader {
    pub bf_signature: [u8; 2],  // 2 bytes: 'BM'
    pub bf_size: u32,      // 4 bytes: file size
    pub bf_reserved1: u16, // 2 bytes: reserved
    pub bf_reserved2: u16, // 2 bytes: reserved
    pub bf_off_bits: u32,  // 4 bytes: offset to image data
}

#[repr(C)] // Ensures the struct layout matches C-style layout
#[derive(Debug, Clone, Copy)]
pub struct BitmapInfoHeader {
    pub bi_size: u32,            // 4 bytes: size of the DIB header (40 bytes)
    pub bi_width: i32,           // 4 bytes: width of the image
    pub bi_height: i32,          // 4 bytes: height of the image
    pub bi_planes: u16,          // 2 bytes: number of planes (1)
    pub bi_bit_depth: u16,       // 2 bytes: bits per pixel (e.g., 24)
    pub bi_compression: u32,     // 4 bytes: compression type
    pub bi_size_image: u32,      // 4 bytes: size of image data
    pub bi_xpels_per_meter: i32, // 4 bytes: horizontal resolution
    pub bi_ypels_per_meter: i32, // 4 bytes: vertical resolution
    pub bi_clr_used: u32,        // 4 bytes: number of colors used
    pub bi_clr_important: u32,   // 4 bytes: important colors
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BitmapHeader {
    pub file_header: BitmapFileHeader,
    pub info_header: BitmapInfoHeader,
}

impl BitmapHeader {
    pub fn new() -> Self {
        BitmapHeader {
            file_header: BitmapFileHeader {
                bf_signature: [b'B', b'M'], // "BM" signature
                bf_size: 0,
                bf_reserved1: 0,
                bf_reserved2: 0,
                bf_off_bits: 54, // Standard offset to pixel data for Windows BMPs
            },
            info_header: BitmapInfoHeader {
                bi_size: 40, // Size of BITMAPINFOHEADER (40 bytes)
                bi_width: 0,
                bi_height: 0,
                bi_planes: 1,
                bi_bit_depth: 24,  // 24 bits per pixel (true color)
                bi_compression: 0, // No compression
                bi_size_image: 0,
                bi_xpels_per_meter: 0,
                bi_ypels_per_meter: 0,
                bi_clr_used: 0,
                bi_clr_important: 0,
            },
        }
    }
}

impl BitmapHeader {
    pub fn new_h(info: BitmapInfoHeader) -> Self {
        BitmapHeader {
            file_header: BitmapFileHeader {
                bf_signature: [b'B', b'M'], // "BM" signature
                bf_size: 0,
                bf_reserved1: 0,
                bf_reserved2: 0,
                bf_off_bits: 54, // Standard offset to pixel data for Windows BMPs
            },
            info_header: BitmapInfoHeader {
                bi_size: info.bi_size, // Size of BITMAPINFOHEADER (40 bytes)
                bi_width: info.bi_width,
                bi_height: info.bi_height,
                bi_planes: info.bi_planes,
                bi_bit_depth: info.bi_bit_depth,  // 24 bits per pixel (true color)
                bi_compression: info.bi_compression, // No compression
                bi_size_image: info.bi_size_image,
                bi_xpels_per_meter: info.bi_xpels_per_meter,
                bi_ypels_per_meter: info.bi_ypels_per_meter,
                bi_clr_used: info.bi_clr_used,
                bi_clr_important: info.bi_clr_important,
            },
        }
    }
}