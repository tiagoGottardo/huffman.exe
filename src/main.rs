use std::{
    fs::File,
    io::{self, BufWriter},
};

use crate::{binary_tree::Node, writer::BitWriter};

pub mod binary_tree;
pub mod writer;

fn main() -> io::Result<()> {
    let path = "input.txt";

    let mut file = File::open(path)?;

    let root = Node::huffman_binary_tree(&mut file)?;

    let file_name = format!("{path}.zip");
    let mut bit_writer = BitWriter::new(BufWriter::new(File::create(file_name)?));

    bit_writer.write(0_u64)?;

    root.write_header(&mut bit_writer)?;

    root.write_body(&mut file, &mut bit_writer)?;

    bit_writer.write_size()?;

    bit_writer.finish()?;

    Ok(())
}
