pub struct PrintStep {
    header_char: char,
    block_char: char,
    v_char: char,
    tl_char: char,
    tr_char: char,
    l_char: char,
    r_char: char,
    bl_char: char,
    br_char: char,
    width: usize,
}

impl PrintStep {
    /// Initialize a new PrintStep with designated width.
    /// 
    /// _Note_: Panics when text being printed is longer than
    /// the `width` variable.
    pub fn new(
        header_char: char,
        tl_char: char,
        tr_char: char,
        l_char: char,
        r_char: char,
        bl_char: char,
        br_char: char,
        v_char: char,
        block_char: char,
        width: usize,
    ) -> Self {
        PrintStep {
            header_char,
            tl_char,
            tr_char,
            l_char,
            r_char,
            bl_char,
            br_char,
            v_char,
            block_char,
            width,
        }
    }

    /// Print header with header char and spacing
    pub fn print_title(&self, text: &str) {
        let padding = self.width - text.len() - 2; // -2 for the border characters
        let left_padding = padding / 2;
        let right_padding = padding - left_padding;

        println!(
            "{}{}{}{}{}",
            self.tl_char,
            self.header_char.to_string().repeat(left_padding),
            text,
            self.header_char.to_string().repeat(right_padding),
            self.tr_char
        );
    }

    /// Print block with block char and spacing
    pub fn print_text(&self, text: &str) {
        let padding = self.width - text.len() - 2; // -2 for the border characters
        let left_padding = padding / 2;
        let right_padding = padding - left_padding;

        println!(
            "{}{}{}{}{}",
            self.v_char,
            self.block_char.to_string().repeat(left_padding),
            text,
            self.block_char.to_string().repeat(right_padding),
            self.v_char
        );
    }

    pub fn print_end(&self, text: &str) {
        let padding = self.width - text.len() - 2; // -2 for the border characters
        let left_padding = padding / 2;
        let right_padding = padding - left_padding;

        println!(
            "{}{}{}{}{}",
            self.bl_char,
            self.header_char.to_string().repeat(left_padding),
            text,
            self.header_char.to_string().repeat(right_padding),
            self.br_char
        );
    }

    pub fn print_break(&self) {
        println!(
            "{}{}{}",
            self.l_char,
            self.header_char.to_string().repeat(self.width - 2),
            self.r_char
        )
    }
}
