use std::alloc::GlobalAlloc;
use std::collections::HashMap;
use std::ops::Deref;
use super::files::*;
use super::huffman_tree::*;

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
pub fn create_huffman_tree(value_frequencies: &Vec<Count>) -> Node {
    let mut nodes: Vec<Node> = Vec::new();
    for frequency in value_frequencies {
        let node = Node::new(frequency.count, Some(frequency.char), None, None);
        nodes.push(node);
    }

    while nodes.len() > 1 {
        // find smallest two nodes according to probability and remove them from Vec
        let node_1 = Box::from(nodes.swap_remove(get_smallest_node_by_index(&nodes)));
        let node_2 = Box::from(nodes.swap_remove(get_smallest_node_by_index(&nodes)));
        // call 'create_upper_node' and add the new node to nodes
        let node = Node::create_upper_node(node_1, node_2);
        nodes.push(node);
    }
    return nodes.pop().unwrap();
}

// creates a Huffman-tree from a bytestream
pub fn create_huffman_tree_from_bytestream(input: &Vec<u8>) -> Node {
    let mut value_frequencies: Vec<Count> = Vec::new();
    for i in 0..input.len() {
        // if value is fresh create new Count else increase 'probs' by 1;
        let mut contains = false;
        for k in 0..value_frequencies.len() {
            if value_frequencies[k].char == input[i] {
                value_frequencies[k].count += 1;
                contains = true;
                break;
            }
        }
        if !contains {
            let count = Count {
                char: input[i],
                count: 1,
            };
            value_frequencies.push(count);
        }
    }
    return create_huffman_tree(&value_frequencies);
}


// takes a Huffman-Tree and returns a Hashmap with <value, huffman-code>
pub fn get_hashmap_for_compression(mut huffman_tree: Box<Node>) -> Box<HashMap<u8, Vec<u8>>> {
    let mut compression_hashmap: Box<HashMap<u8, Vec<u8>>> = Box::new(HashMap::new());
    let mut huffman_code:Vec<u8> = Vec::new();
    rec_hashmap_for_compression(&mut Some(huffman_tree), &mut compression_hashmap, &mut huffman_code);
    return compression_hashmap;
}
fn rec_hashmap_for_compression(node: &mut Option<Box<Node>>, mut compression_hashmap: &mut Box<HashMap<u8, Vec<u8>>>, mut current_huffman_code: &mut Vec<u8>) {
    if node.is_some() {
        if node.as_deref_mut().unwrap().value.is_some() {
            compression_hashmap.insert(node.as_deref_mut().unwrap().value.unwrap(), current_huffman_code.clone());
        }
        else{
            if node.as_deref_mut().unwrap().left.is_some() {
                current_huffman_code.push(1);
                rec_hashmap_for_compression(&mut node.as_deref_mut().unwrap().left, compression_hashmap, current_huffman_code);
                current_huffman_code.pop();
            }
            if node.as_deref_mut().unwrap().right.is_some() {
                current_huffman_code.push(0);
                rec_hashmap_for_compression(&mut node.as_deref_mut().unwrap().right, compression_hashmap, current_huffman_code);
                current_huffman_code.pop();
            }
        }
    }
}


// takes a the header of an comp-file and returns a Hashmap with <huffman-code, value>
pub fn get_hashmap_for_decompression(header: Vec<u8>) -> HashMap<Vec<u8>, u8> {
    let mut decompression_hashmap:HashMap<Vec<u8>, u8> = HashMap::new();

    return decompression_hashmap;
}


// header format: 1Byte: value; 1Byte: len_in_bits_of_huffman-code; 1-(32):Bytes for huffman-code
pub fn get_huffman_code_header_for_file(huffman_code: HashMap<u8, Vec<u8>>) -> Vec<HuffmanMapping> {
    let mut header:Vec<HuffmanMapping> = Vec::new();

    return header;
}