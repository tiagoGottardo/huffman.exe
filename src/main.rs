use std::{
    fs::File,
    io::{self, BufWriter},
};

use crate::{
    binary_tree::{Node, push_arbitrary_bin},
    writer::BitWriter,
};

pub mod binary_tree;
pub mod writer;

fn main() -> io::Result<()> {
    let path = "input.txt";

    let mut file = File::open(path)?;

    let root = Node::huffman_binary_tree(&mut file)?;

    let header = root.mount_tree_header();

    let body = root.mount_body(&mut file)?;

    let unused_bits = (3 + header.len() + body.len()) % 8;

    let mut bits_vec: Vec<bool> = vec![];
    push_arbitrary_bin(&mut bits_vec, unused_bits, 3);

    let file_name = format!("{path}.zip");
    let mut bit_writer = BitWriter::new(BufWriter::new(File::create(file_name)?));

    bit_writer.write(bits_vec)?;
    bit_writer.write(header)?;
    bit_writer.write(body)?;

    bit_writer.finish()?;

    Ok(())
}
