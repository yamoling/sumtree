"""This file contains the SumTree class definition"""
from typing import Tuple, List

__version__: str

class SumTree:
    """
    SumTree class
    A SumTree is a binary tree in which the value of a node is the sum of its direct children.
    As such, only leaves retain useful information.
    """

    def __init__(self, capacity: int):
        """"""
    def add(self, value: float):
        """Add an item to the sumtree"""
    def get(self, cumsum: float) -> Tuple[int, float]:
        """Retrieve the index and the value of the leaf corresponding to the given sumlative tum"""
    def update(self, leaf_num: int, value: float):
        """Update the value of the leaf with a new value"""
    def update_batched(self, leaf_nums: List[int], values: List[float]):
        """Update the values of the leaves with new values. The two lists must have the same length"""
    def sample(self, n_samples: int) -> Tuple[List[int], List[float]]:
        """
        Randomly sample `n_samples` leaves.

        Every leaf has a probability proportional to its value to be sampled.
        The same leaf could be sampled multiple times.
        """
    def sample_batched(self, n_samples: int) -> Tuple[List[int], List[float]]:
        """
        Sample from the tree by splitting the tree value into `n_samples` batches.
        Every leaf in that range has a probability proportional to its value to be sampled.

        E.g: if tree.total is 60 and n = 3, one leaf will be selected in
        [0, 20), one in [20, 40) and one in [40, 60)
        """
        def is_empty(self) -> bool:
            """Whether the tree is empty"""
    def seed(self, seed_value: int):
        """Seed the Random Number Generator for sampling"""
    def __getitem__(self, leaf_num: int) -> float:
        """
        Retrieve the value of the leaf at the given (positive) index.
        """
    def __len__(self) -> int: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...
    def __getstate__(self): ...
    def __setstate__(self, state): ...
    @property
    def total(self) -> float:
        """The total value of the tree, i.e. the cumulative sum of all leaves"""
    @property
    def capacity(self) -> int:
        """The capacity of the tree (i.e. the number of leaves)"""
