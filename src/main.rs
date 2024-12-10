use bitmap_header::{BitmapFileHeader, BitmapHeader};
use bitwise_util::{u32_to_u8_arr, u32_to_u8_arr_experimental};
use file_utils::FileEditor;
use print_step::PrintStep;

mod bitmap_header;
mod bitwise_util;
mod file_utils;
mod print_step;

fn main() {
    // let ps = PrintStep::new('─', ' ', 40);

    // let fh: FileEditor = FileEditor::new("test.bmp");

    // let bmh = BitmapHeader::new_h(bitmap_header::BitmapInfoHeader {
    //     bi_size: (40),
    //     bi_width: (0xff),
    //     bi_height: (0xff),
    //     bi_planes: (0x01),
    //     bi_bit_depth: (0x08),
    //     bi_compression: (0x00),
    //     bi_size_image: (0xff),
    //     bi_xpels_per_meter: (0xff),
    //     bi_ypels_per_meter: (0xff),
    //     bi_clr_used: (0xff),
    //     bi_clr_important: (0x00),
    // });
    // ps.print_title("FILE HEADER GENERATION STARTING");
    // fh.write_bytes(&bmh.file_header.bf_signature);
    // fh.write_u8(&bmh.file_header.bf_size);

    let u: u32 = 0x89abcdef;
    u32_to_u8_arr_experimental(u);
    println!(
        "{:08x?} : {:02x?} : {:02x?}",
        u,
        u32_to_u8_arr(u),
        u32_to_u8_arr_experimental(u)
    )
}
