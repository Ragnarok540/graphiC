

#[cfg(test)]
mod tests {

    use crate::funcs::rand_permutation;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_rand_permutation() {
        let vec = crate::funcs::rand_permutation(4);
        println!("{:?}", vec);
        assert_eq!(vec.len(), 4);
    }
}

mod funcs {

    extern crate rand;
    use rand::Rng;

    pub fn rand_permutation(n: usize) -> Vec<usize> {
        let mut vec = Vec::new();
        for i in 0..n {
            vec.push(i + 1);
        }
        let mut rng = rand::thread_rng();
        for i in 0..n {
            let j = rng.gen_range(0, n);
            let k = vec[i];
            vec[i] = vec[j];
            vec[j] = k;
        }
        vec
    }
}
