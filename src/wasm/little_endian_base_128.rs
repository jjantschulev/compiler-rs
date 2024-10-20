pub trait EncodesToLeb128 {
    /// Encode the value to the provided output buffer.
    /// Returns the number of bytes written.
    fn encode_to_leb128(&self, output: &mut Vec<u8>) -> usize;

    /// Encode the value to a new `Vec<u8>`.
    fn encode_to_leb128_bytes(&self) -> Vec<u8> {
        let mut output = Vec::new();
        self.encode_to_leb128(&mut output);
        output
    }
}

impl EncodesToLeb128 for u32 {
    fn encode_to_leb128(&self, output: &mut Vec<u8>) -> usize {
        let mut value = *self;
        let mut bytes_written = 0;
        loop {
            if value < 0x80 {
                output.push(value as u8);
                bytes_written += 1;
                break bytes_written;
            } else {
                output.push((value & 0x7F) as u8 | 0x80);
                value >>= 7;
            }
        }
    }
}

impl EncodesToLeb128 for usize {
    fn encode_to_leb128(&self, output: &mut Vec<u8>) -> usize {
        let mut value = *self;
        let mut bytes_written = 0;
        loop {
            if value < 0x80 {
                output.push(value as u8);
                bytes_written += 1;
                break bytes_written;
            } else {
                output.push((value & 0x7F) as u8 | 0x80);
                value >>= 7;
            }
        }
    }
}

impl EncodesToLeb128 for u64 {
    fn encode_to_leb128(&self, output: &mut Vec<u8>) -> usize {
        let mut value = *self;
        let mut bytes_written = 0;
        loop {
            if value < 0x80 {
                output.push(value as u8);
                bytes_written += 1;
                break bytes_written;
            } else {
                output.push((value & 0x7F) as u8 | 0x80);
                value >>= 7;
            }
        }
    }
}

impl EncodesToLeb128 for i32 {
    fn encode_to_leb128(&self, output: &mut Vec<u8>) -> usize {
        let mut value = *self;
        let mut bytes_written = 0;
        loop {
            let mut byte = (value & 0x7F) as u8;
            value >>= 7; // Arithmetic right shift for signed integers

            if value == 0 && byte & 0x40 == 0 || value == -1 && byte & 0x40 != 0 {
                output.push(byte);
                bytes_written += 1;
                break bytes_written;
            }

            byte |= 0x80;
            output.push(byte);
            bytes_written += 1;
        }
    }
}

impl EncodesToLeb128 for i64 {
    fn encode_to_leb128(&self, output: &mut Vec<u8>) -> usize {
        let mut value = *self;
        let mut bytes_written = 0;
        loop {
            let mut byte = (value & 0x7F) as u8;
            value >>= 7; // Arithmetic right shift for signed integers

            if value == 0 && byte & 0x40 == 0 || value == -1 && byte & 0x40 != 0 {
                output.push(byte);
                bytes_written += 1;
                break bytes_written;
            }

            byte |= 0x80;
            output.push(byte);
            bytes_written += 1;
        }
    }
}
