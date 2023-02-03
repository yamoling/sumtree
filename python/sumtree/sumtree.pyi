"""This file contains the SumTree class definition"""
from typing import Tuple


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

    @property
    def total(self) -> float:
        """The total value of the tree, i.e. the cumulative sum of all leaves"""

    @property
    def capacity(self) -> int:
        """The capacity of the tree (i.e. the number of leaves)"""
