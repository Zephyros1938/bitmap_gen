pub struct PrintStep {
    header_char: char,
    block_char: char,
    width: usize,
}

impl PrintStep {
    /// Initialize a new PrintStep with designated width,
    /// header character, and block character
    pub fn new(header_char: char, block_char: char, width: usize) -> Self {
        PrintStep {
            header_char,
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
            "┌{}{}{}┐",
            self.header_char.to_string().repeat(left_padding),
            text,
            self.header_char.to_string().repeat(right_padding)
        );
    }

    /// Print block with block char and spacing
    pub fn print_block(&self, text: &str) {
        let padding = self.width - text.len() - 2; // -2 for the border characters
        let left_padding = padding / 2;
        let right_padding = padding - left_padding;

        println!(
            "┌{}{}{}┐",
            self.block_char.to_string().repeat(left_padding),
            text,
            self.block_char.to_string().repeat(right_padding)
        );
    }
}
