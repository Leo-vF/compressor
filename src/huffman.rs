use super::files::*;
use super::huffman_tree::*;

pub fn create_huffman_tree(value_frequency:Vec<Count>) -> Option<Node> {
    let mut nodes: Vec<Node>;
    for prob in value_frequency {
        let node = Node::new(prob.count, Some(prob.char), None, None);
        nodes.push(node);
    }

    while nodes.len() > 1 {
        // find smallest two nodes according to probability and remove them from Vec
        let mut smallest_prop = (0, 0);
        for i in 0..nodes.len(){
            if nodes[i].probability < nodes[smallest_prop.0].probability {
                smallest_prop.1 = smallest_prop.0;
                smallest_prop.0 = i;
            }
            else if nodes[i].probability < nodes[smallest_prop.1].probability {
                smallest_prop.1 = i;
            }
        }
        // call 'create_upper_node' and add the new node to nodes
        let node = Node::create_upper_node(Box::from(nodes.swap_remove(remove(smallest_prop.0))), Box::from(nodes.swap_remove(smallest_prop.1)));
        nodes.push(node);
    }
    return nodes.pop();
}
