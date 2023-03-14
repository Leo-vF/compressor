
pub struct Node{
    value: Option<u8>,
    probability: u32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}
impl Node {
    pub fn new(probability: u32, value: Option<u8>, left: Option<Box<Node>>, right: Option<Box<Node>>) -> Self{
        Self{
            value,
            probability,
            left,
            right,
        }
    }
    pub fn create_upper_node(src_left: Box<Node>, src_right: Box<Node>) -> Self{
        Self {
            value: None,
            probability: src_left.probability + src_right.probability,
            left: Some(src_left),
            right: Some(src_right),
        }
    }
}