use bitmap_header::{BitmapFileHeader, BitmapHeader};
use bitwise_util::{u32_to_u8_arr, u32_to_u8_arr_experimental};
use file_utils::FileEditor;
use print_step::PrintStep;

mod bitmap_header;
mod bitwise_util;
mod file_utils;
mod print_step;

fn main() {
    let ps = PrintStep::new('─', '┌', '┐', '├', '┤', '└', '┘', '│', ' ', 75);

    let mut fh: FileEditor = FileEditor::new("test.bmp");

    let size_x: u32 = 0xff;
    let size_y: u32 = 0xff;

    let mut bitmap = BitmapHeader::new_h(bitmap_header::BitmapInfoHeader {
        size: (54),
        width: (size_x),
        height: (size_y),
        planes: (0x01),
        bit_depth: (0x08),
        compression: (0x00),
        px_per_m: (0xff),
        px_per_m_x: (0xff),
        px_per_m_y: (0xff),
        palette: (0xff),
        important_cols: (0x00),
    });
    bitmap.header.size =
        bitmap.info.width * bitmap.info.height * (bitmap.info.bit_depth / 8) as u32;
    bitmap.header.size += (bitmap.h_size + bitmap.inf_size) as u32;
    println!(
        "Total Size (Bytes) : 0x{:08x?}\nTotal Size (Bits)  : 0b{:b}",
        bitmap.header.size, bitmap.header.size
    );
    /*
       Size calculation for bitmap header = 14 + 40 + (width * height * (bitdepth/8))
    */
    ps.print_title("FILE HEADER GENERATION STARTING");
    ps.print_text("Total Size (Bytes)");
    ps.print_text(&format!("{:08x?}", bitmap.header.size).to_string());
    ps.print_text("Total Size (Bits)");
    ps.print_text(&format!("{:b}", bitmap.header.size).to_string());
    ps.print_break();
    ps.print_text("testing");
    ps.print_end("");
    fh.write_bytes(&bitmap.header.signature);
    fh.write_u32(bitmap.header.size);
    fh.write_u32(bitmap.header.reserved);
    fh.write_u32(bitmap.header.offset);
}
