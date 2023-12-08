use pyo3::{
    prelude::{pyclass, pymethods},
    types::PyDict,
    PyResult,
};
use rand::{rngs::StdRng, Rng, SeedableRng};

#[pyclass(module = "sumtree")]
#[derive(Clone)]
/// SumTree class
/// A SumTree is a binary tree in which the value of a node is the sum of its direct children.
/// As such, only leaves retain useful information.
pub struct SumTree {
    /// The number of leaves in the SumTree
    n_leaves: usize,
    /// The tree nodes
    tree: Vec<f64>,
    /// Number of items (leaves) in the tree
    num_items: usize,
    /// The maximal number of items (leaves) in the tree
    capacity: usize,
    /// First leaf index
    first_leaf: usize,
    /// Next index to write
    write_index: usize,
    rng: StdRng,
}

#[pymethods]
impl SumTree {
    #[new]
    pub fn new(capacity: usize) -> Self {
        let num_nodes = 2 * capacity - 1;

        SumTree {
            n_leaves: capacity,
            tree: vec![0f64; num_nodes],
            num_items: 0,
            first_leaf: capacity - 1,
            capacity,
            write_index: 0,
            rng: StdRng::seed_from_u64(rand::random()),
        }
    }

    /// The total cumulative sum
    #[getter]
    pub fn total(&self) -> f64 {
        self.tree[0]
    }

    /// The maximal number of items (leaves) that the tree can store
    #[getter]
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn add(&mut self, value: f64) {
        self.update(self.write_index, value);
        self.write_index = (self.write_index + 1) % self.capacity;
        self.num_items = std::cmp::min(self.capacity, self.num_items + 1)
    }

    /// Update the SumTree by changing a leaf value.
    /// The change is propagated up to the root.
    pub fn update(&mut self, leaf_num: usize, value: f64) {
        let mut index = leaf_num + self.n_leaves - 1;
        let delta = value - self.tree[index];
        while index > 0 {
            self.tree[index] += delta;
            index = (index - 1) / 2;
        }
        self.tree[0] += delta;
    }

    pub fn update_batched(&mut self, leaf_nums: Vec<usize>, values: Vec<f64>) {
        assert_eq!(leaf_nums.len(), values.len());
        for (&leaf_num, &value) in leaf_nums.iter().zip(values.iter()) {
            self.update(leaf_num, value);
        }
    }

    /// Get the leaf number and leaf value that corresponds
    /// to the given cumulative sum.
    pub fn get(&self, mut cumsum: f64) -> (usize, f64) {
        let mut idx = 0;
        while idx < self.first_leaf {
            let left = 2 * idx + 1;
            if cumsum <= self.tree[left] {
                // Left child
                idx = left;
            } else {
                // Right child
                idx = left + 1;
                cumsum -= self.tree[left];
            }
        }
        // Can only return the highest leaf (num_items - 1)
        let leaf_num = idx - self.first_leaf;
        // let leaf_num = std::cmp::min(idx - self.first_leaf, self.num_items - 1);
        let value = self.tree[idx];
        (leaf_num, value)
    }

    /// Randomly sample `n_samples` leaves. Every leaf has a probability proportional
    /// to its value to be sampled.
    /// The same leaf could be sampled multiple times.
    pub fn sample(&mut self, n_samples: usize) -> (Vec<usize>, Vec<f64>) {
        let total = self.total();
        let mut indices = vec![];
        let mut values = vec![];
        for _ in 0..n_samples {
            let cumsum = self.rng.gen::<f64>() * total;
            let (index, value) = self.get(cumsum);
            indices.push(index);
            values.push(value);
        }
        (indices, values)
    }

    /// Sample from the tree by splitting the tree value into `n_samples` batches.
    /// If tree.value is 60 and n = 3, one leaf will be selected in
    /// [0, 20), in [20, 40) and one in [40, 60)
    pub fn sample_batched(&mut self, n_samples: usize) -> (Vec<usize>, Vec<f64>) {
        let batch_size = self.total() / (n_samples as f64);
        let mut indices = vec![];
        let mut values = vec![];
        let mut lower_bound = 0f64;
        for _ in 0..n_samples {
            let leaf_value = self.rng.gen::<f64>() * batch_size + lower_bound;
            let (index, value) = self.get(leaf_value);
            indices.push(index);
            values.push(value);
            lower_bound += batch_size
        }
        (indices, values)
    }

    pub fn seed(&mut self, seed_value: u64) {
        self.rng = StdRng::seed_from_u64(seed_value);
    }

    pub fn is_empty(&self) -> bool {
        self.num_items == 0
    }

    pub fn __deepcopy__(&self, _memo: &PyDict) -> Self {
        self.clone()
    }

    pub fn __len__(&self) -> usize {
        self.num_items
    }

    pub fn __getitem__(&self, leaf_num: usize) -> PyResult<f64> {
        if let Some(value) = self.tree.get(leaf_num + self.first_leaf) {
            return Ok(*value);
        }
        Err(pyo3::exceptions::PyIndexError::new_err(format!(
            "Index out of bounds: trying to access index {} but there are only {} leaves",
            leaf_num, self.n_leaves
        )))
    }

    pub fn __setitem__(&mut self, leaf_num: usize, value: f64) {
        self.update(leaf_num, value);
    }

    pub fn __repr__(&self) -> String {
        // If the float values hold on 6 chars (12.345) + 1 whitespace
        // Leading '[ ' + trailing ']' = 3 chars
        let mut res = String::with_capacity(self.n_leaves * 7 + 3);
        res.push_str(&format!(
            "SumTree(capacity={}, total={:.3}, [ ",
            self.capacity,
            self.total()
        ));
        for i in self.first_leaf..(self.first_leaf + self.num_items) {
            res.push_str(&format!("{:.3} ", self.tree[i]))
        }
        res.push_str("])");
        res
    }

    /// Pickle protocol
    pub fn __getstate__(&self) -> PyResult<(Vec<f64>, usize)> {
        Ok((self.tree.clone(), self.num_items))
    }

    /// Pickle protocol
    pub fn __setstate__(&mut self, state: (Vec<f64>, usize)) -> PyResult<()> {
        let (tree, n_items) = state;
        self.tree = tree;
        self.num_items = n_items;
        Ok(())
    }

    /// Pickle protocol
    pub fn __getnewargs__(&self) -> PyResult<(usize,)> {
        Ok((self.capacity,))
    }
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
        assert_eq!(index, 3);
        assert_eq!(value, 0.);
    }

    #[test]
    fn sumtree_get() {
        let mut st = SumTree::new(4);
        st.add(20.);
        st.add(20.);
        st.add(20.);
        st.add(20.);
        for (i, cumsum) in (0..80).step_by(20).enumerate() {
            let (index, value) = st.get(cumsum as f64);
            assert_eq!(value, 20.);
            if i == 0 {
                assert_eq!(index, 0);
            } else {
                assert_eq!(index, i - 1);
            }
        }
        let (index, value) = st.get(80.);
        assert_eq!(value, 20.);
        assert_eq!(index, 3);
    }

    #[test]
    fn sumtree_get_above_cumsum() {
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
    fn sumtree_get_below_min() {
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
    fn sumtree_get_plenty() {
        use rand::random;
        let mut st = SumTree::new(50_000);
        for _ in 0..1_000_000 {
            st.add(random());
            let cumsum: f64 = random::<f64>() * st.total();
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

    #[test]
    fn test_seed_equal() {
        let mut st1 = SumTree::new(50_000);
        let mut st2 = SumTree::new(50_000);
        st1.seed(0);
        st2.seed(0);
        for i in 0..1_000 {
            st1.add(i as f64);
            st2.add(i as f64);
        }
        let (idx1, _) = st1.sample(20);
        let (idx2, _) = st2.sample(20);
        assert_eq!(idx1, idx2)
    }

    #[test]
    fn test_seed_different() {
        let mut st1 = SumTree::new(50_000);
        let mut st2 = SumTree::new(50_000);
        st1.seed(0);
        st2.seed(1);
        for i in 0..1_000 {
            st1.add(i as f64);
            st2.add(i as f64);
        }
        let (idx1, _) = st1.sample(20);
        let (idx2, _) = st2.sample(20);
        assert_ne!(idx1, idx2)
    }

    #[test]
    fn test_no_seed_different() {
        let mut st1 = SumTree::new(50_000);
        let mut st2 = SumTree::new(50_000);
        for i in 0..1_000 {
            st1.add(i as f64);
            st2.add(i as f64);
        }
        let (idx1, _) = st1.sample(20);
        let (idx2, _) = st2.sample(20);
        assert_ne!(idx1, idx2)
    }

    #[test]
    fn test_update_batched() {
        let mut st = SumTree::new(8);
        for i in 0..8 {
            st.add(i as f64);
        }

        for (i, cumsum) in vec![0, 1, 3, 6, 10, 15, 21, 28].into_iter().enumerate() {
            let (index, value) = st.get(cumsum as f64);
            assert_eq!(index, i);
            assert_eq!(value, i as f64);
        }
        let (index, value) = st.get(300f64);
        assert_eq!(index, 7);
        assert_eq!(value, 7f64);

        // Update the 4 first leaves the the value 5f64.
        st.update_batched(Vec::from_iter(0..4), vec![5f64; 4]);

        let expected_leaf_values = [5f64, 5f64, 5f64, 5f64, 4f64, 5f64, 6f64, 7f64];
        for (i, cumsum) in vec![2, 7, 12, 17, 24, 29, 35, 300].into_iter().enumerate() {
            let (index, value) = st.get(cumsum as f64);
            assert_eq!(index, i);
            assert_eq!(value, expected_leaf_values[i]);
        }
    }
}
