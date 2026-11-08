use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    fs::{File, read_to_string},
    io::{self, BufRead, BufReader, BufWriter},
};

use crate::{
    binary_tree::{Node, push_arbitrary_bin},
    writer::BitWriter,
};

pub mod binary_tree;
pub mod writer;

fn main() -> io::Result<()> {
    let mut hs: HashMap<char, usize> = HashMap::new();

    let all_content = read_to_string("input.txt")?;

    all_content.chars().for_each(|c| {
        hs.entry(c).and_modify(|f| *f += 1).or_insert(1);
    });

    let mut min_heap: BinaryHeap<Reverse<Node>> = BinaryHeap::new();

    hs.iter().for_each(|(k, v)| {
        min_heap.push(Reverse(Node::new(*v).with_char(*k)));
    });

    let mut heap_keys = min_heap.iter().count();

    let root = {
        while heap_keys >= 2 {
            let (n, n2) = (min_heap.pop(), min_heap.pop());

            if let (Some(Reverse(n)), Some(Reverse(n2))) = (n, n2) {
                let (left, right) = if n.value <= n2.value {
                    (n, n2)
                } else {
                    (n2, n)
                };

                let new_node = Node {
                    key: None,
                    value: left.value + right.value,
                    left: Some(Box::new(left)),
                    right: Some(Box::new(right)),
                };

                min_heap.push(Reverse(new_node));
            }

            heap_keys -= 1;
        }

        let Reverse(root) = min_heap.pop().expect("Should have one node");

        root
    };

    let hs = root.hash_map();

    hs.iter()
        .for_each(|(k, (v0, v1))| println!("{k}: {:0width$b}", v0, width = v1));

    let mut header = root.mount_tree_header();

    header
        .iter()
        .for_each(|b| print!("{}", if *b { 1 } else { 0 }));

    let mut body: Vec<bool> = vec![];

    let reader = BufReader::new(File::open("input.txt")?);

    for line_result in reader.lines() {
        line_result?.chars().for_each(|c| {
            hs.get(&c)
                .map(|(num, width)| push_arbitrary_bin(&mut body, *num, *width));
        });
    }

    println!();

    body.iter()
        .for_each(|b| print!("{}", if *b { 1 } else { 0 }));

    let unused_bits = (3 + header.len() + body.len()) % 8;

    let mut bits_vec: Vec<bool> = vec![];

    push_arbitrary_bin(&mut bits_vec, unused_bits, 3);
    bits_vec.append(&mut header);
    bits_vec.append(&mut body);

    let mut bit_writer = BitWriter::new(BufWriter::new(File::create("input.zip")?));

    bit_writer.write(bits_vec)?;

    Ok(())
}
