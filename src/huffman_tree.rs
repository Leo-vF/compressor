pub struct Node {
    pub value: Option<u8>,
    pub value_frequency: u32,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}
impl Node {
    pub fn new(probability: u32, value: Option<u8>) -> Self {
        Self {
            value,
            value_frequency: probability,
            left: None,
            right: None,
        }
    }
    pub fn create_upper_node(src_left: Box<Node>, src_right: Box<Node>) -> Self {
        Self {
            value: None,
            value_frequency: src_left.value_frequency + src_right.value_frequency,
            left: Some(src_left),
            right: Some(src_right),
        }
    }
}
