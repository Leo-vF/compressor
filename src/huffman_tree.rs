pub struct Node {
    pub value: Option<u8>,
    pub value_frequency: u32,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}
impl Node {
    pub fn new(value_frequency: u32, value_src: u8) -> Self {
        Self {
            value: Some(value_src),
            value_frequency,
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
