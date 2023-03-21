use super::files::*;
use super::hashing::*;
use super::huffman_tree::*;
use std::collections::HashMap;
use std::ops::Deref;

fn get_smallest_node_by_index(nodes: &Vec<Node>) -> usize {
    let mut smallest_node = 0;
    for i in 0..nodes.len() {
        if nodes[i].value_frequency < nodes[smallest_node].value_frequency {
            smallest_node = i;
        }
    }
    return smallest_node;
}

// takes a &Vec<Count> and returns a Huffman-tree
fn create_huffman_tree(nodes: &mut Vec<Node>) -> Node {
    while nodes.len() > 1 {
        // find smallest two nodes according to probability and remove them from Vec
        let node_1 = Box::from(nodes.swap_remove(get_smallest_node_by_index(&nodes)));
        let node_2 = Box::from(nodes.swap_remove(get_smallest_node_by_index(&nodes)));
        // call 'create_upper_node' and add the new node to nodes

        let node: Node = Node::create_upper_node(node_1, node_2);
        nodes.push(node);
    }
    return nodes.pop().unwrap();
}
fn analyse_bytestream(
    dest: &mut HashMap<u8, Node, BuildCompressionHasher>,
    data: &Vec<u8>,
    index_from: usize,
    index_to: usize,
) {
    for i in index_from..index_to {
        match dest.get_mut(&data[i]) {
            None => {
                dest.insert(data[i], Node::new(1, data[i]));
            }
            Some(node) => {
                node.value_frequency += 1;
            }
        }
    }
}

// creates a Huffman-tree from a bytestream
fn create_huffman_tree_from_bytestream(input: &Vec<u8>) -> Node {
    let mut value_frequencies: HashMap<u8, Node, BuildCompressionHasher> =
        HashMap::with_hasher(BuildCompressionHasher);
    analyse_bytestream(&mut value_frequencies, input, 0, input.len());

    return create_huffman_tree(
        &mut value_frequencies
            .into_iter()
            .map(|(_id, node)| node)
            .collect(),
    );
}

// takes a Huffman-Tree and returns an Array  with 'value' as index and Vec<huffman-codes> as value
fn get_huffman_code_resolver_for_compression(data: &Vec<u8>) -> Box<[Vec<u8>; 257]> {
    let huffman_tree = create_huffman_tree_from_bytestream(&data);

    const INIT_VALUE: Vec<u8> = Vec::new();
    let mut huffman_codes: Box<[Vec<u8>; 257]> = Box::new([INIT_VALUE; 257]);
    let mut current_huffman_code: Vec<u8> = Vec::new();
    rec_huffman_codes_for_compression(
        &mut Some(&huffman_tree),
        &mut huffman_codes,
        &mut current_huffman_code,
    );
    return huffman_codes;
}

fn rec_huffman_codes_for_compression(
    node: &Option<&Node>,
    huffman_codes: &mut Box<[Vec<u8>; 257]>,
    current_huffman_code: &mut Vec<u8>,
) {
    match &node {
        None => {}
        Some(node) => {
            match node.value {
                None => {
                    // node is not a leaf
                    match &node.left {
                        // check if there is a left node
                        None => {}
                        Some(left) => {
                            // add a '0' to the huffman-code and go int the next deeper left-node
                            current_huffman_code.push(0);
                            rec_huffman_codes_for_compression(
                                &mut Some(left.deref()),
                                huffman_codes,
                                current_huffman_code,
                            );
                            current_huffman_code.pop();
                        }
                    }
                    match &node.right {
                        None => {}
                        Some(right) => {
                            current_huffman_code.push(1);
                            rec_huffman_codes_for_compression(
                                &mut Some(right),
                                huffman_codes,
                                current_huffman_code,
                            );
                            current_huffman_code.pop();
                        }
                    }
                }
                Some(value) => {
                    // check if there is only one u8 to be encoded
                    if current_huffman_code.len() == 0 {
                        current_huffman_code.push(1);
                        huffman_codes[value as usize] = current_huffman_code.to_owned();
                        huffman_codes[256].push(value);
                        current_huffman_code.pop();
                    } else {
                        // create huffman_code for the found value
                        huffman_codes[value as usize] = current_huffman_code.to_owned();
                        huffman_codes[256].push(value);
                    }
                }
            }
        }
    }
}

// takes a the header of an comp-file and returns a Hashmap with <huffman-code, value>
fn get_hashmap_for_decompression(mappings: Vec<HuffmanMapping>) -> Box<HashMap<Vec<u8>, u8>> {
    let mut decompression_hashmap: Box<HashMap<Vec<u8>, u8>> = Box::new(HashMap::new());
    for mut huffman_mapping in mappings {
        let mut encoding: Vec<u8> = Vec::new();
        let number_of_bits_in_last_byte = (huffman_mapping.len_of_encoding
            - (huffman_mapping.encoding.len() - 1) as u8 * 8)
            as u8;
        while huffman_mapping.encoding.len() > 1 {
            let tmp = huffman_mapping.encoding.remove(0);
            for k in 0..8 {
                encoding.push((tmp >> (7 - k)) & 0x1);
            }
        }
        let tmp = huffman_mapping.encoding.remove(0);
        for k in 0..number_of_bits_in_last_byte {
            encoding.push((tmp >> (7 - k)) & 0x1);
        }

        decompression_hashmap.insert(encoding, huffman_mapping.char);
    }

    return decompression_hashmap;
}

// header format: 1Byte: value; 1Byte: len_in_bits_of_huffman-code; 1-(32):Bytes for huffman-code
fn get_huffman_code_header_for_file(huffman_codes: Box<[Vec<u8>; 257]>) -> Vec<HuffmanMapping> {
    let mut mappings: Vec<HuffmanMapping> = Vec::new();
    for key in &huffman_codes[256] {
        let tmp = &huffman_codes[*key as usize];

        let length_of_encoding = 1 + ((tmp.len() as isize - 1) / 8) as usize;
        let mut encoding: Vec<u8> = Vec::new();
        for i in 0..length_of_encoding - 1 {
            let mut encoding_byte: u8 = 0;
            for k in 0..8 {
                encoding_byte += tmp.get((i * 8) + k).unwrap() << (7 - k);
            }
            encoding.push(encoding_byte);
        }
        let mut last_encoding_byte: u8 = 0;
        let mut last_byte_offset = tmp.len() % 8;
        if last_byte_offset == 0 {
            last_byte_offset = 8;
        }
        for k in 0..last_byte_offset {
            last_encoding_byte += tmp.get(((length_of_encoding - 1) * 8) + k).unwrap() << (7 - k);
        }
        encoding.push(last_encoding_byte);
        let mapping = HuffmanMapping::new(*key, tmp.len() as u8, encoding);
        mappings.push(mapping);
    }
    return mappings;
}

pub fn compress(data: Vec<u8>) -> File {
    let huffman_codes = get_huffman_code_resolver_for_compression(&data);
    let mut compressed_data: Vec<u8> = Vec::new();

    let mut compressed_byte: u8 = 0;
    let mut pos_in_byte: u8 = 7; // bit offset in byte
    for byte in data {
        for bit in &huffman_codes[byte as usize] {
            compressed_byte += bit << pos_in_byte;
            if pos_in_byte == 0 {
                pos_in_byte = 7;
                compressed_data.push(compressed_byte);
                compressed_byte = 0;
            } else {
                pos_in_byte -= 1;
            }
        }
    }

    // calc bit offset for header
    pos_in_byte = (pos_in_byte + 1) % 8;
    if pos_in_byte != 0 {
        compressed_data.push(compressed_byte);
    }

    let mappings = get_huffman_code_header_for_file(huffman_codes);
    let compressed_file: File = File {
        mappings,
        data: compressed_data,
        last_byte_offset: pos_in_byte,
    };
    return compressed_file;
}

fn get_longest(map: &Box<HashMap<Vec<u8>, u8>>) -> usize {
    let mut max = 0;
    for i in map.keys() {
        if i.len() > max {
            max = i.len();
        }
    }
    return max;
}
pub fn decompress(compressed_file: File) -> Vec<u8> {
    let mut data: Vec<u8> = Vec::new();
    let huffman_code_to_byte = get_hashmap_for_decompression(compressed_file.mappings);
    let mut huffman_code: Vec<u8> = Vec::new();
    let mut _compressed_byte: u8 = 0;
    for i in 0..compressed_file.data.len() - 1 {
        _compressed_byte = *compressed_file.data.get(i).unwrap();
        for k in 0..8 {
            huffman_code.push((_compressed_byte >> 7 - k) & 0x1);
            let data_byte;
            match huffman_code_to_byte.get(&*huffman_code) {
                None => {}
                Some(byte) => {
                    data_byte = *byte;
                    data.push(data_byte);
                    huffman_code.clear();
                }
            }
            if huffman_code.len() > length_of_longest_huffman_code {
                panic!("ERROR! Huffman invalid!");
            }
        }
    }
    _compressed_byte = *compressed_file
        .data
        .get(compressed_file.data.len() - 1)
        .unwrap();
    for k in 0..(8 - compressed_file.last_byte_offset) {
        huffman_code.push((_compressed_byte >> 7 - k) & 0x1);
        if huffman_code_to_byte.contains_key(&*huffman_code) {
            data.push(*huffman_code_to_byte.get(&*huffman_code).unwrap());
            huffman_code.clear();
        }
    }
    return data;
}
