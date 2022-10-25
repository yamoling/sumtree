use pyo3::prelude::{pymodule, pyclass, pymethods, Python, PyResult, PyModule};


#[pyclass]
/// SumTree class
/// A SumTree is a binary tree in which the value of a node is the sum of its direct children.
/// As such, only leaves retain useful information.
pub struct SumTree {
    /// The number of leaves in the SumTree
    n_leaves: usize,
    /// The tree nodes
    tree: Vec<f32>,
    /// Number of items (leaves) in the tree
    num_items: usize,
    /// The maximal number of items (leaves) in the tree
    capacity: usize,
    /// First leaf index
    first_leaf: usize
}


#[pymethods]
impl SumTree {
    #[new]
    pub fn new(capacity: usize) -> Self {
        let num_nodes = 2 * capacity -1;
        SumTree { 
            n_leaves: capacity, 
            tree: vec![0f32; num_nodes],
            num_items: 0,
            first_leaf: capacity - 1,
            capacity
        }
    }

    /// The total cumulative sum
    #[getter]
    pub fn total(&self) -> f32 {
        self.tree[0]
    }

    /// The maximal number of items (leaves) that the tree can store
    #[getter]
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    fn is_leaf(&self, index: usize) -> bool {
        return index >= self.n_leaves - 1;
    }

    pub fn __len__(&self) -> usize {
        self.num_items
    }

    pub fn add(&mut self, value: f32) {
        self.update(self.num_items % self.capacity, value);
        self.num_items = std::cmp::min(self.capacity, self.num_items + 1)
    }

    /// Update the SumTree by changing a leaf value.
    /// The change is propagated up to the root.
    fn update(&mut self, leaf_num: usize, value: f32) {
        let mut index = leaf_num + self.n_leaves - 1;
        let delta = value - self.tree[index];
        while index > 0 {
            self.tree[index] += delta;
            index = (index - 1) / 2;
        }
        self.tree[0] += delta;
    }

    /// Get the leaf number and leaf value that corresponds
    /// to the given cumulative sum.
    pub fn get(&self, mut cumsum: f32) -> (usize, f32) {
        let mut idx = 0;
        while idx < self.first_leaf {
            let left = 2 * idx + 1;
            if cumsum < self.tree[left]{
                // Left child
                idx = left;
            } else {
                // Right child
                idx = left + 1;
                cumsum = cumsum - self.tree[left];
            }
        }
        // Can only return the highest leaf (num_items - 1)
        let leaf_num = std::cmp::min(idx - self.first_leaf, self.num_items - 1);
        let value = self.tree[idx];
        (leaf_num, value)
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn sumtree(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<SumTree>()?;
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_ok() {
        let st = SumTree::new(8);
        assert_eq!(st.n_leaves, 8);
        assert_eq!(st.total(), 0.);
        assert_eq!(st.num_items, 0);
        assert_eq!(st.capacity(), 8);
    }

    #[test]
    fn sumtree_total() {
        let mut st = SumTree::new(4);
        assert_eq!(st.total(), 0.);
        st.add(20.);
        assert_eq!(st.total(), 20.);
        st.add(20.);
        assert_eq!(st.total(), 40.);
        st.add(20.);
        assert_eq!(st.total(), 60.);
        st.add(20.);
        assert_eq!(st.total(), 80.);
        st.add(10.);
        assert_eq!(st.total(), 70.);
    }

    #[test]
    fn sumtree_get_empty() {
        let st = SumTree::new(4);
        let (index, value) = st.get(50.);
        assert_eq!(index, 0);
        assert_eq!(value, 0.);
    }

    #[test]
    fn sumtree_get(){
        let mut st = SumTree::new(4);
        st.add(20.);
        st.add(20.);
        st.add(20.);
        st.add(20.);
        for (i, cumsum) in (0..80).step_by(20).enumerate() {
            let (index, value) = st.get(cumsum as f32);
            assert_eq!(value, 20.);
            assert_eq!(index, i);
        }
        let (index, value) = st.get(80.);
        assert_eq!(value, 20.);
        assert_eq!(index, 3);
    }


    #[test]
    fn sumtree_get_above_cumsum(){
        let mut st = SumTree::new(4);
        st.add(20.);
        st.add(20.);
        st.add(20.);
        st.add(20.);
        let (index, value) = st.get(100000.);
        assert_eq!(value, 20.);
        assert_eq!(index, 3);
    }

    #[test]
    fn sumtree_get_below_min(){
        let mut st = SumTree::new(4);
        st.add(20.);
        st.add(20.);
        st.add(20.);
        st.add(20.);
        let (index, value) = st.get(-100000.);
        assert_eq!(value, 20.);
        assert_eq!(index, 0);
    }

    #[test]
    fn sumtree_get_plenty(){
        use rand::random;
        let mut st = SumTree::new(50_000);
        for _ in 0..10000000 {
            st.add(random());
            let cumsum: f32 = random::<f32>() * st.total();
            let (index, _) = st.get(cumsum);
            assert!(index < st.num_items);
            assert!(index < 50_000);
        }
    }

    #[test]
    fn sumtree_get_exactly_tree_value(){
        use rand::random;
        let mut st = SumTree::new(50_000);
        for _ in 0..100000 {
            st.add(random());
        }
    }
}
