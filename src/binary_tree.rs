use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap},
    fs::File,
    io::Write,
    io::{self, Read, Seek},
};

use crate::writer::BitWriter;

const CAPACITY: usize = 8 * 1024;

#[derive(Debug)]
pub struct Node {
    pub key: Option<char>,
    pub value: usize,
    pub right: Option<Box<Node>>,
    pub left: Option<Box<Node>>,
}

impl Node {
    pub fn new(value: usize) -> Self {
        Self {
            key: None,
            value,
            right: None,
            left: None,
        }
    }

    pub fn with_char(mut self, key: char) -> Self {
        self.key = Some(key);

        self
    }

    pub fn push(&mut self, node: Node) {
        if node.value > self.value {
            if let Some(ref mut right) = self.right {
                right.push(node);
            } else {
                self.right = Some(Box::new(node));
            }

            return;
        }

        if let Some(ref mut left) = self.left {
            left.push(node);
        } else {
            self.left = Some(Box::new(node));
        }
    }

    pub fn write_header<W: Write + Seek>(&self, bit_writer: &mut BitWriter<W>) -> io::Result<()> {
        if let Some(key) = self.key {
            bit_writer.write(true)?;

            if !key.is_ascii() {
                panic!("The char {key} is not ascii!");
            };

            bit_writer.write(key as u8)?;
        } else {
            bit_writer.write(false)?;
        }

        if let Some(ref left) = self.left {
            left.write_header(bit_writer)?;
        }

        if let Some(ref right) = self.right {
            right.write_header(bit_writer)?;
        }

        Ok(())
    }

    pub fn write_body<W: Write + Seek>(
        &self,
        file: &mut File,
        bit_writer: &mut BitWriter<W>,
    ) -> io::Result<()> {
        let hash_map = self.hash_map();
        let mut block = [0_u8; CAPACITY];

        loop {
            let read_bytes = file.read(&mut block)?;

            if read_bytes == 0 {
                break;
            }

            for byte in block.iter() {
                if let Some((num, width)) = hash_map.get(&(*byte as char)) {
                    bit_writer.write((*num, *width))?;
                }
            }
        }

        Ok(())
    }

    pub fn hash_map(&self) -> HashMap<char, (usize, usize)> {
        let mut hs = HashMap::new();

        if let Some(key) = self.key {
            hs.insert(key, (0, 1));

            return hs;
        }

        if let Some(ref left) = self.left {
            left.hash_map_aux(&mut hs, (0, 1));
        }

        if let Some(ref right) = self.right {
            right.hash_map_aux(&mut hs, (1, 1));
        }

        hs
    }

    fn hash_map_aux(&self, hs: &mut HashMap<char, (usize, usize)>, code: (usize, usize)) {
        if let Some(key) = self.key {
            hs.insert(key, code);

            return;
        }

        if let Some(ref left) = self.left {
            let (num, bits) = code;

            left.hash_map_aux(hs, (num << 1, bits + 1));
        }

        if let Some(ref right) = self.right {
            let (num, bits) = code;

            right.hash_map_aux(hs, ((num << 1) + 1, bits + 1));
        }
    }

    pub fn huffman_binary_tree(file: &mut File) -> io::Result<Self> {
        let mut freq: HashMap<char, usize> = HashMap::new();

        let mut block = [0_u8; CAPACITY];

        loop {
            let read_bytes = file.read(&mut block)?;

            if read_bytes == 0 {
                break;
            }

            for byte in block.iter() {
                freq.entry(*byte as char)
                    .and_modify(|f| *f += 1)
                    .or_insert(1);
            }
        }

        file.rewind()?;

        let mut min_heap: BinaryHeap<Reverse<Node>> = BinaryHeap::new();

        freq.iter().for_each(|(k, v)| {
            min_heap.push(Reverse(Node::new(*v).with_char(*k)));
        });

        while min_heap.len() >= 2 {
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
        }

        let Reverse(root) = min_heap.pop().expect("Should have one node");

        Ok(root)
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for Node {}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

pub fn push_arbitrary_bin(vec: &mut Vec<bool>, input: usize, width: usize) {
    for shift in 1..=width {
        vec.push((input >> (width - shift)) & 1 == 1);
    }
}
