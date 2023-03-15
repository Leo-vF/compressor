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

pub fn create_huffman_tree(value_frequencies: &Vec<Count>) -> Option<Node> {
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
    return nodes.pop();
}

pub fn create_huffman_tree_from_bytestream(input: &Vec<u8>) -> Option<Node> {
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

