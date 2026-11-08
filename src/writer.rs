use std::io::{self, Write};

pub struct BitWriter<W> {
    out: W,
    byte: u8,
    used: u8,
}

impl<W: Write> BitWriter<W> {
    pub fn new(out: W) -> Self {
        Self {
            out,
            byte: 0,
            used: 0,
        }
    }

    pub fn write_bit(&mut self, bit: bool) -> io::Result<()> {
        self.byte = (self.byte << 1) | u8::from(bit);
        self.used += 1;

        if self.used == 8 {
            self.out.write_all(&[self.byte])?;
            self.byte = 0;
            self.used = 0;
        }
        Ok(())
    }

    pub fn write(&mut self, vec: Vec<bool>) -> io::Result<()> {
        for bit in vec.iter() {
            self.write_bit(*bit)?;
        }

        Ok(())
    }

    pub fn write_arbitrary_bin_size(&mut self, input: usize, size: usize) -> io::Result<()> {
        for shift in 1..=size {
            self.write_bit((input >> (size - shift)) & 1 == 1)?;
        }

        Ok(())
    }

    pub fn finish(mut self) -> io::Result<W> {
        if self.used > 0 {
            self.byte <<= 8 - self.used; // zeros à direita
            self.out.write_all(&[self.byte])?;
        }
        Ok(self.out)
    }
}
