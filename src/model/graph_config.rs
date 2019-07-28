pub struct GraphConfig {
    pub simple: bool,
    pub directed: bool,
    pub acyclic: bool,
    pub weighted: bool,
    pub min_weight: usize,
    pub max_weight: usize,
}

impl GraphConfig {

    pub fn new(simple: bool, 
               directed: bool,
               acyclic: bool,
               weighted: bool,
               min_weight: usize,
               max_weight: usize) -> GraphConfig {

        GraphConfig {
            simple,
            directed,
            acyclic,
            weighted,
            min_weight,
            max_weight,
        }

    }

}
