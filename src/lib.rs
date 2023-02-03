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
    first_leaf: usize,
    /// Next index to write
    write_index: usize
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
            capacity,
            write_index: 0
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

    pub fn add(&mut self, value: f32) {
        self.update(self.write_index, value);
        self.write_index = (self.write_index + 1) % self.capacity;
        self.num_items = std::cmp::min(self.capacity, self.num_items + 1)
    }

    /// Update the SumTree by changing a leaf value.
    /// The change is propagated up to the root.
    pub fn update(&mut self, leaf_num: usize, value: f32) {
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
            if cumsum <= self.tree[left]{
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

    /// Randomly sample `n_samples` leaves. Every leaf has a probability proportional
    /// to its value to be sampled.
    /// The same leaf could be sampled multiple times.
    pub fn sample(&self, n_samples: usize) -> (Vec<usize>, Vec<f32>) {
        let total = self.total();
        let mut indices = vec![];
        let mut values = vec![];
        for _ in 0..n_samples {
            let (index, value) = self.get(rand::random::<f32>() * total);
            indices.push(index);
            values.push(value);
        }
        (indices, values)
    }

    /// Sample from the tree by splitting the tree value into `n_samples` batches.
    /// If tree.value is 60 and n = 3, one leaf will be selected in
    /// [0, 20), in [20, 40) and one in [40, 60)
    pub fn sample_batched(&self, n_samples: usize) -> (Vec<usize>, Vec<f32>) {
        let batch_size = self.total() / (n_samples as f32);
        let mut indices = vec![];
        let mut values = vec![];
        let mut lower_bound = 0f32;
        for _ in 0..n_samples {
            let leaf_value = rand::random::<f32>() * batch_size + lower_bound;
            let (index, value) = self.get(leaf_value);
            indices.push(index);
            values.push(value);
            lower_bound += batch_size
        }
        (indices, values)
    }


    pub fn __len__(&self) -> usize {
        self.num_items
    }

    pub fn __getitem__(&self, leaf_num: usize) -> f32 {
        self.tree[self.first_leaf + leaf_num]
    }

    pub fn __str__(&self) -> String {
        // If the float values hold on 6 chars (12.345) + 1 whitespace
        // Leading '[ ' + trailing ']' = 3 chars
        let mut res = String::with_capacity(self.n_leaves * 7 + 3);
        res.push_str("[ ");
        for i in self.first_leaf..(self.first_leaf + self.num_items) {
            res.push_str(&format!("{:.3} ", self.tree[i]))
        }
        res.push(']');
        res
    }

    pub fn __repr__(&self) -> String {
        self.__str__()
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
        for _ in 0..1_000_000 {
            st.add(random());
            let cumsum: f32 = random::<f32>() * st.total();
            let (index, _) = st.get(cumsum);
            assert!(index < st.num_items);
            assert!(index < 50_000);
        }
    }

    #[test]
    fn sumtree_sample() {
        let mut st = SumTree::new(50_000);
        for _ in 0..10 {
            st.add(1.);
        }
        let (indices, _) = st.sample(20);
        assert!(indices.len() == 20)
    }

    #[test]
    fn sumtree_sample_batched() {
        let mut st = SumTree::new(50_000);
        for _ in 0..10 {
            st.add(1.);
        }
        let (indices, _) = st.sample_batched(20);
        for (i, tree_idx) in indices.iter().enumerate() {
            assert!(2 * tree_idx <= i)
        }
    }
    
}
