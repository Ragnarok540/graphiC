pub struct Graph {
    pub node_i: Vec<usize>,
    pub node_j: Vec<usize>,
    pub weight: Vec<usize>,
}

impl Graph {

    pub fn new() -> Graph {

        Graph {
            node_i: Vec::new(),
            node_j: Vec::new(),
            weight: Vec::new(),
        }

    }

}
