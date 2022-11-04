//! An Union Find data structure [Union Find Code - Youtuber WilliamFiset](https://www.youtube.com/watch?v=KbFlZYCpONw).

/// An union find data struture.
#[derive(Debug)]
pub struct UnionFind {

    /// The number of elements in this union find, must be bigger than 0.
    size: usize,

    /// Used to track the sizes of each components.
    sizes: Vec<usize>,

    /// id[i] points to the parent of node i, if id[i] == i then node i is a root node
    id: Vec<usize>,

    /// Tracks the number of components in the union find.
    num_components: usize,

}

#[derive(Debug)]
pub enum UnionFindError{
    UnionFindSizeError,
    UnionFindInvalidId,
}

impl UnionFind {
    /// Create a new union find data structure.
    /// 
    /// # Error
    /// 
    /// Raise error if size <= 0;
    pub fn new(size :usize) -> Result<UnionFind, UnionFindError> {
        if size <= 0 {
            return Err(UnionFindError::UnionFindSizeError);
        }

        Ok(UnionFind { 
            size, 
            sizes: vec![1; size], 
            id: (0..size).into_iter().collect(), 
            num_components: size 
        })
    }

    /// Find the root node of the node `p`.
    /// 
    /// # Error
    /// 
    /// Error if node p is not valid, aka not in the range of the indices of the node.
    pub fn find(&mut self, mut p: usize) -> Result<usize, UnionFindError> {
        let id = &mut self.id;
        if p >= self.size {
            return Err(UnionFindError::UnionFindInvalidId);
        }

        // find the root of p
        let mut root = p;
        while root != id[root] {
            root = id[root]
        }

        // let point to the root node all the nodes along the path from p to root.
        while p != root {
            let next = id[p];
            id[p] = root;
            p = next;
        }

        Ok(root)
    }

    /// Return whether or not the nodes `p` and `q` are in the same components.
    /// 
    /// # Error
    /// 
    /// Error if `p` or `q` is not valid, aka not in the range of the indices of the node.
    pub fn connected(&mut self, p: usize, q: usize) -> Result<bool, UnionFindError> {
        Ok(self.find(p)? == self.find(q)?)
    }

    /// Return the number of the nodes in this union find.
    pub fn size(&self) -> usize {
        self.size
    }

    /// Return the number of the components in this union find.
    pub fn components_size(&self) -> usize {
        self.num_components
    }

    /// Unify the components containing nodes `p` and `q`
    /// 
    /// # Error
    /// 
    /// Error if `p` or `q` is not valid, aka not in the range of the indices of the node.
    pub fn unify(&mut self, p: usize, q: usize) -> Result<(), UnionFindError> {
        let root_p = self.find(p)?;
        let root_q = self.find(q)?;

        let sz = &mut self.sizes;
        let id = &mut self.id;

        // if `p` and `q` are not in the same component.
        if root_p != root_q {
            // Merge smaller componet into the larger one.
            if sz[root_p] < sz[root_q] {
                sz[root_q] += sz[root_p];
                id[root_p] = root_q;
            } else {
                sz[root_q] += sz[root_p];
                id[root_p] = root_q;
            }
        }

        Ok(())
    }



}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn unionfind_new() {
        for size in 1 as usize ..2 {
            let uf = UnionFind::new(size).expect("Should create a new union find data structure");
            assert_eq!(size, uf.size);
            assert_eq!(size, uf.num_components);
            
            assert_eq!(size, uf.sizes.len());
            for sz in uf.sizes.iter() {
                assert_eq!(1, *sz);
            }

            assert_eq!(size, uf.id.len());
            let mut count = 0;
            for id in uf.id.iter() {
                assert_eq!(count, *id);
                count = count + 1;
            }
        }
        if let Err(UnionFindError::UnionFindSizeError) = UnionFind::new(0) {
            
        } else {
            panic!();
        }
    }

    #[test]
    fn unionf_find() {
    }

}