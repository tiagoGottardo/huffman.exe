use std::io::{self, Seek, SeekFrom, Write};

pub struct BitWriter<W> {
    out: W,
    byte: u8,
    used: u8,
    total_bits: u64,
}

impl<W: Write + Seek> BitWriter<W> {
    pub fn new(out: W) -> Self {
        Self {
            out,
            byte: 0,
            used: 0,
            total_bits: 0,
        }
    }

    pub fn write_bit(&mut self, bit: bool) -> io::Result<()> {
        self.byte = (self.byte << 1) | u8::from(bit);
        self.used += 1;
        self.total_bits += 1;

        if self.used == 8 {
            self.out.write_all(&[self.byte])?;
            self.byte = 0;
            self.used = 0;
        }
        Ok(())
    }

    pub fn write_size(&mut self) -> io::Result<()> {
        let current_position = self.out.stream_position()?;

        self.out.rewind()?;
        self.out.write_all(&self.total_bits.to_be_bytes())?;
        self.out.seek(SeekFrom::Start(current_position))?;

        Ok(())
    }

    pub fn write<T: BitWritable>(&mut self, value: T) -> io::Result<()> {
        value.write_to(self)
    }

    pub fn finish(mut self) -> io::Result<W> {
        if self.used > 0 {
            self.byte <<= 8 - self.used; // zeros à direita
            self.out.write_all(&[self.byte])?;
        }

        self.out.flush()?;
        Ok(self.out)
    }
}

pub trait BitWritable {
    fn write_to<W: Write + Seek>(self, bit_writer: &mut BitWriter<W>) -> io::Result<()>;
}

impl BitWritable for bool {
    fn write_to<W: Write + Seek>(self, bit_writer: &mut BitWriter<W>) -> io::Result<()> {
        bit_writer.write_bit(self)?;
        Ok(())
    }
}

impl BitWritable for (usize, usize) {
    fn write_to<W: Write + Seek>(self, bit_writer: &mut BitWriter<W>) -> io::Result<()> {
        for shift in (0..self.1).rev() {
            let bit = ((self.0 >> shift) & 1) != 0;
            bit_writer.write_bit(bit)?;
        }

        Ok(())
    }
}

impl BitWritable for Vec<bool> {
    fn write_to<W: Write + Seek>(self, bit_writer: &mut BitWriter<W>) -> io::Result<()> {
        for e in self.into_iter() {
            bit_writer.write_bit(e)?;
        }
        Ok(())
    }
}

macro_rules! impl_bit_writable {
    ($($t:ty),*) => {
        $(
            impl BitWritable for $t {
                fn write_to<W: Write + Seek>(self, bit_writer: &mut BitWriter<W>) -> io::Result<()> {
                    for shift in (0..Self::BITS).rev() {
                        let bit = ((self >> shift) & 1) != 0;
                        bit_writer.write_bit(bit)?;
                    }

                    Ok(())
                }
            }
        )*
    };
}

impl_bit_writable!(u8, u64, usize);
