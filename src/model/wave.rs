pub struct Wave {
    pub width: u32,
    pub height: u32,
    pub indices: Vec<Vec<usize>>,
    pub last_index_collapsed: usize,
}
