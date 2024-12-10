#[repr(C)] // Ensures the struct layout matches C-style layout
#[derive(Debug, Clone, Copy)]
pub struct BitmapFileHeader {
    pub signature: [u8; 2],  // 2 bytes: 'BM'
    pub size: u32,      // 4 bytes: file size
    pub reserved: u32, // 4 bytes: reserved
    pub offset: u32,  // 4 bytes: offset to image data
}

#[repr(C)] // Ensures the struct layout matches C-style layout
#[derive(Debug, Clone, Copy)]
pub struct BitmapInfoHeader {
    pub size: u32,            // 4 bytes: size of the DIB header (40 bytes)
    pub width: u32,           // 4 bytes: width of the image
    pub height: u32,          // 4 bytes: height of the image
    pub planes: u16,          // 2 bytes: number of planes (1)
    pub bit_depth: u16,       // 2 bytes: bits per pixel (e.g., 24)
    pub compression: u32,     // 4 bytes: compression type
    pub px_per_m: u32,      // 4 bytes: size of image data
    pub px_per_m_x: u32, // 4 bytes: horizontal resolution
    pub px_per_m_y: u32, // 4 bytes: vertical resolution
    pub palette: u32,        // 4 bytes: number of colors used
    pub important_cols: u32,   // 4 bytes: important colors
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BitmapHeader {
    pub header: BitmapFileHeader,
    pub info: BitmapInfoHeader,
    pub h_size: u8,
    pub inf_size: u8
}

impl BitmapHeader {
    pub fn new() -> Self {
        BitmapHeader {
            header: BitmapFileHeader {
                signature: [b'B', b'M'], // "BM" signature
                size: 0,
                reserved: 0,
                offset: 54, // Standard offset to pixel data for Windows BMPs
            },
            info: BitmapInfoHeader {
                size: 40, // Size of BITMAPINFOHEADER (40 bytes)
                width: 0,
                height: 0,
                planes: 1,
                bit_depth: 24,  // 24 bits per pixel (true color)
                compression: 0, // No compression
                px_per_m: 0,
                px_per_m_x: 0,
                px_per_m_y: 0,
                palette: 0,
                important_cols: 0,
            },
            h_size: 14,
            inf_size: 40
        }
    }
}

impl BitmapHeader {
    pub fn new_h(info: BitmapInfoHeader) -> Self {
        BitmapHeader {
            header: BitmapFileHeader {
                signature: [b'B', b'M'], // "BM" signature
                size: 0,
                reserved: 0,
                offset: 54, // Standard offset to pixel data for Windows BMPs
            },
            info: BitmapInfoHeader {
                size: info.size, // Size of BITMAPINFOHEADER (40 bytes)
                width: info.width,
                height: info.height,
                planes: info.planes,
                bit_depth: info.bit_depth,  // 24 bits per pixel (true color)
                compression: info.compression, // No compression
                px_per_m: info.px_per_m,
                px_per_m_x: info.px_per_m_x,
                px_per_m_y: info.px_per_m_y,
                palette: info.palette,
                important_cols: info.important_cols,
            },
            h_size: 14,
            inf_size: 40
        }
    }
}