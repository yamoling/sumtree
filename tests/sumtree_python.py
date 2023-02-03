from dataclasses import dataclass
from typing import List, Tuple

@dataclass
class SumTree:
    """
    SumTree, a binary tree data structure where the parent's value is the sum of its children
    """
    _n_leaves: int
    """The maximal number of leaves"""
    _tree: List[float]
    """The tree itself, represented as an array"""
    _write_index: int
    """The next index to write the data"""

    def __init__(self, capacity: int):
        self._n_leaves = capacity
        self._tree =  [0.] *  (2 * capacity - 1)
        self._write_index = 0

    def _is_leaf(self, idx: int) -> bool:
        """Whether the given tree index is a leaf"""
        return idx >= self._n_leaves - 1

    def _tree_idx_to_leaf_num(self, tree_idx: int) -> int:
        """Transforms a tree index into a leaf number"""
        leaf_idx = tree_idx - self._n_leaves + 1
        assert 0 <= leaf_idx < self._n_leaves, f"The given index {tree_idx} is not a leaf!"
        return leaf_idx

    def _leaf_num_to_tree_idx(self, leaf_num: int) -> int:
        """Transforms a leaf number to a tree index"""
        tree_idx = leaf_num + self._n_leaves - 1
        assert 0 <= tree_idx < self._n_leaves * 2 - 1
        return tree_idx

    @property
    def capacity(self):
        return self._n_leaves

    @property
    def total(self):
        """Cumulative sum"""
        return self._tree[0]

    def add(self, value: float):
        """Add an item to the TreeSum"""
        self.update(self._write_index, value)
        self._write_index = (self._write_index + 1) % self._n_leaves

    def update(self, leaf_num: int, value: float):
        """Update the value of the leaf whose number is given as argument"""
        idx = leaf_num + self._n_leaves - 1
        delta = value - self._tree[idx]
        while idx >= 0:
            self._tree[idx] += delta
            # Get the parent index
            idx = (idx - 1) // 2

    def get(self, cumsum: float) -> Tuple[int, float]:
        """
        Get the index and the value of the leaf corresponding to the given cumsum.
        """
        num_nodes = len(self._tree)
        idx = 0
        left = 2 * idx + 1
        while left < num_nodes:
            if cumsum <= self._tree[left]:
                # Left child
                idx = left
            else:
                # Right child
                idx = left + 1
                cumsum = cumsum - self._tree[left]
            left = 2 * idx + 1
        leaf_num = idx - self._n_leaves + 1
        return leaf_num, self._tree[idx]


    def __repr__(self) -> str:
        return f"SumTree (sum = {self.total}, nodes={self._tree})"
