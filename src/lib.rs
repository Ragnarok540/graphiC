extern crate matrix;
extern crate rand;

mod model;

use matrix::prelude::*;
use rand::Rng;
use model::graph::*;

pub fn rand_permutation(n: usize) -> Vec<usize> {
    let mut vec = Vec::new();

    for i in 0..n {
        vec.push(i + 1);
    }

    let mut rng = rand::thread_rng();

    for i in 0..n {
        let j = rng.gen_range(0, n);
        vec.swap(i, j);
    }

    vec
}

pub fn random_graph(usize nodes, 
                    usize edges, 
                    GraphConfig config) -> Graph {

    let mut adj_matrix = Conventional::zero(edges, edges);

    let mut max_edges = 0;

    if config.simple {
        max_edges = edges * (edges - 1);
        if !config.directed {
            max_edges /= 2;
        }
        if edges > max_edges {
            panic!("too many edges");
        }
    }

//    let graph = Graph::new(); 
//    graph

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rand_permutation() {
        let vec = rand_permutation(4);
        assert_eq!(vec.len(), 4);
        let sum = vec.iter().fold(0, |acc, x| acc + x);
        assert_eq!(sum, 10);
    }

    #[test]
    fn test_random_graph() {
        let _graph = random_graph();
        assert_eq!(2 + 2, 4);
    }
}
