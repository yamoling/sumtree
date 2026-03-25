import random
from copy import deepcopy

import sumtree as strust
import sumtree_python as stp


def test_version():
    # Retrieve the version from the cargo.toml file
    import toml

    with open("Cargo.toml", "r") as f:
        cargo = toml.load(f)
    assert cargo["package"]["version"] == strust.__version__


def test_basic_sumtree_comparison():
    st1 = stp.SumTree(1000)
    st2 = strust.SumTree(1000)
    for i in range(100_000):
        r = random.randint(0, 100)
        st1.add(r)
        st2.add(r)
        assert st1.total == st2.total, f"{st1.total}, {st2.total}"

        to_get = random.randint(0, int(st2.total))
        g1 = st1.get(to_get)
        g2 = st2.get(to_get)
        assert g1 == g2, f"i={i}\t{g1} != {g2}"

        new_value = random.randint(0, 1000)
        st1.update(i % st1.capacity, new_value)
        st2.update(i % st2.capacity, new_value)


def test_deepcopy():
    st = strust.SumTree(1000)
    for i in range(1000):
        st.add(i)
    st2 = deepcopy(st)
    assert st.total == st2.total
    assert st.capacity == st2.capacity

    st.add(50)
    assert st.total == st2.total + 50.0


def test_pickle():
    import pickle

    st = strust.SumTree(1000)
    st.add(42)

    serialized = pickle.dumps(st)
    st2 = pickle.loads(serialized)
    assert st.total == st2.total
    assert st.capacity == st2.capacity


def test_str():
    st = strust.SumTree(1000)
    st.add(42.0)
    assert str(st) == "SumTree(capacity=1000, total=42.000, [ 42.000 ])"


def test_update_batched():
    st = strust.SumTree(8)
    for _ in range(8):
        st.add(1)
    assert st.total == 8
    st.update_batched([0, 1, 2, 3], [2, 2, 2, 2])
    assert st.total == 4 * 2 + 4 * 1


def test_indexing():
    st = strust.SumTree(8)
    for i in range(8):
        st.add(i)
    for i in range(8):
        assert st[i] == i


def test_wrong_indexing():
    st = strust.SumTree(8)
    try:
        st[25]
    except IndexError:
        pass

    try:
        st[-1]
    except OverflowError:
        pass


def test_update_batched_wrong_shape():
    st = strust.SumTree(8)
    for i in range(8):
        st.add(i)

    try:
        st.update_batched([0, 1], [10])  # Mismatched lengths
        assert False, "Should raise ValueError for mismatched lengths"
    except ValueError:
        pass


# ============================================================================
# EMPTY AND ZERO VALUE TESTS
# ============================================================================


def test_add_zero_values():
    """Test adding zero values to the tree."""
    st = strust.SumTree(10)
    st.add(0)
    st.add(0)
    st.add(5)
    assert st.total == 5.0


def test_update_to_zero():
    """Test updating a value to zero."""
    st = strust.SumTree(10)
    st.add(5)
    assert st.total == 5.0
    st.update(0, 0)
    assert st.total == 0.0


def test_update_from_zero():
    """Test updating a zero value to non-zero."""
    st = strust.SumTree(10)
    st.add(0)
    st.add(0)
    assert st.total == 0.0
    st.update(0, 10)
    assert st.total == 10.0


# ============================================================================
# BOUNDARY AND CAPACITY TESTS
# ============================================================================


def test_capacity_boundaries():
    """Test operations at exact capacity."""
    capacity = 100
    st = strust.SumTree(capacity)
    for i in range(capacity):
        st.add(1)
    assert st.total == float(capacity)


def test_fill_tree_completely():
    """Test filling tree to exact capacity."""
    st = strust.SumTree(5)
    for i in range(5):
        st.add(i + 1)
    assert st.total == 15.0
    for i in range(5):
        assert st[i] == i + 1


def test_index_at_boundaries():
    """Test accessing first and last indices."""
    st = strust.SumTree(10)
    for i in range(10):
        st.add(i)

    assert st[0] == 0
    assert st[9] == 9


def test_update_at_boundaries():
    """Test updating first and last indices."""
    st = strust.SumTree(10)
    for i in range(10):
        st.add(10)

    original_total = st.total
    st.update(0, 20)
    assert st.total == original_total + 10.0

    st.update(9, 5)
    assert st.total == original_total + 10.0 - 5.0


# ============================================================================
# EDGE CASES
# ============================================================================
def test_get_from_empty():
    st = strust.SumTree(5)
    _, value = st.get(0)
    assert value == 0

    _, value = st.get(-5)
    assert value == 0


def test_get_exact_total():
    """Test get with value equal to total."""
    st = strust.SumTree(5)
    for i in range(1, 6):
        st.add(i)

    total = st.total
    idx, _ = st.get(total)
    assert idx < len(st) - 1


def test_get_distribution_correctness():
    """Test that get returns indices with correct probability distribution."""
    st = strust.SumTree(3)
    st.add(10.0)
    st.add(20.0)
    st.add(30.0)

    counts = [0, 0, 0]
    samples = 10000
    for _ in range(samples):
        idx, _ = st.get(random.uniform(0, st.total))
        counts[idx] += 1

    # Expected counts: [1667, 3333, 5000] approximately
    # Allow 20% variance
    assert 1300 < counts[0] < 2000, f"Index 0 count: {counts[0]}"
    assert 2700 < counts[1] < 4000, f"Index 1 count: {counts[1]}"
    assert 4200 < counts[2] < 5800, f"Index 2 count: {counts[2]}"


# ============================================================================
# STATE CONSISTENCY TESTS
# ============================================================================


def test_total_consistency_after_operations():
    """Test that total remains valid after mixed operations."""
    st = strust.SumTree(20)
    expected_total = 0.0

    # Add operations
    for i in range(10):
        st.add(i)
        expected_total += i

    assert st.total == expected_total

    # Update operations
    st.update(0, 100)
    expected_total = expected_total - 0 + 100
    assert st.total == expected_total

    st.update(5, 50)
    expected_total = expected_total - 5 + 50
    assert st.total == expected_total


def test_repeated_updates_same_index():
    """Test multiple updates to the same index."""
    st = strust.SumTree(10)
    st.add(100)

    values = [50, 75, 25, 150, 10]
    for v in values:
        st.update(0, v)
        assert st.total == float(v)
        assert st[0] == v


def test_alternating_add_update():
    """Test alternating add and update operations."""
    st = strust.SumTree(10)
    expected_total = 0.0

    for i in range(5):
        st.add(i + 1)
        expected_total += i + 1
        assert st.total == expected_total

        st.update(i, (i + 1) * 2)
        expected_total = expected_total - (i + 1) + (i + 1) * 2
        assert st.total == expected_total


def test_float_values():
    """Test operations with float values."""
    st = strust.SumTree(10)
    values = [1.5, 2.7, 3.14, 0.001, 999.999]

    total = 0.0
    for v in values:
        st.add(v)
        total += v

    assert abs(st.total - total) < 1e-9


def test_large_values():
    """Test very large number handling."""
    st = strust.SumTree(10)
    large_vals = [1e6, 1e7, 1e8, 1e9, 1e10]

    total = 0.0
    for v in large_vals:
        st.add(v)
        total += v

    assert abs(st.total - total) < total * 1e-9


def test_mixed_int_float():
    """Test mixed integer and float operations."""
    st = strust.SumTree(10)
    st.add(10)
    st.add(20.5)
    st.add(30)
    st.add(40.75)

    assert st.total == 101.25


# ============================================================================
# BATCHED OPERATION TESTS
# ============================================================================


def test_update_batched_empty_list():
    """Test batched update with empty lists."""
    st = strust.SumTree(10)
    for i in range(10):
        st.add(i)

    original_total = st.total
    st.update_batched([], [])
    assert st.total == original_total


def test_update_batched_single_element():
    """Test batched update with one element."""
    st = strust.SumTree(10)
    for i in range(10):
        st.add(i)

    st.update_batched([5], [100])
    assert st[5] == 100


def test_update_batched_all_elements():
    """Test batched update of all indices."""
    st = strust.SumTree(5)
    for i in range(5):
        st.add(i)

    st.update_batched([0, 1, 2, 3, 4], [10, 10, 10, 10, 10])
    assert st.total == 50.0


def test_update_batched_to_zero():
    """Test batched update with zero values."""
    st = strust.SumTree(8)
    for i in range(8):
        st.add(5)

    st.update_batched([0, 2, 4, 6], [0, 0, 0, 0])
    assert st.total == 20.0


def test_update_batched_overlapping_indices():
    """Test batched update with non-overlapping indices."""
    st = strust.SumTree(10)
    for i in range(10):
        st.add(1)

    st.update_batched([1, 3, 5, 7, 9], [2, 2, 2, 2, 2])
    assert st.total == 5 * 2 + 5 * 1


def test_update_batched_preserves_consistency():
    """Test that batched updates maintain consistency."""
    st = strust.SumTree(10)
    for i in range(10):
        st.add(i)

    expected_total = sum(range(10))
    st.update_batched([0, 1, 2], [100, 200, 300])
    expected_total = expected_total - 0 - 1 - 2 + 100 + 200 + 300
    assert st.total == expected_total


# ============================================================================
# INDEXING EDGE CASES
# ============================================================================


def test_index_zero():
    """Test accessing index 0."""
    st = strust.SumTree(10)
    st.add(42)
    assert st[0] == 42


def test_index_negative_various():
    """Test various negative indices."""
    st = strust.SumTree(10)
    for i in range(10):
        st.add(i)

    for neg_idx in [-1, -5, -10]:
        try:
            st[neg_idx]
            assert False, "Should raise OverflowError for negative index"
        except OverflowError:
            pass


def test_index_out_of_capacity_range():
    """Test indices beyond capacity."""
    st = strust.SumTree(5)
    for i in range(5):
        st.add(i)

    for out_idx in [5, 10, 100, 1000]:
        try:
            st[out_idx]
            assert False, f"Should raise IndexError for index {out_idx}"
        except IndexError:
            pass


def test_get_all_indices():
    """Test accessing each valid index sequentially."""
    st = strust.SumTree(20)
    for i in range(20):
        st.add(i)

    for i in range(20):
        assert st[i] == i


# ============================================================================
# STRESS AND CONSISTENCY TESTS
# ============================================================================


def test_many_small_adds():
    """Test many small additions."""
    st = strust.SumTree(1000)
    expected_total = 0.0

    for i in range(1000):
        st.add(0.1)
        expected_total += 0.1

    assert abs(st.total - expected_total) < 0.01


def test_few_large_adds():
    """Test few large additions."""
    st = strust.SumTree(10)
    values = [1e6, 2e6, 3e6, 4e6, 5e6]

    expected_total = 0.0
    for v in values:
        st.add(v)
        expected_total += v

    assert abs(st.total - expected_total) < expected_total * 1e-9


def test_constant_total_updates():
    """Test updates that maintain total value."""
    st = strust.SumTree(10)
    for i in range(10):
        st.add(10)

    original_total = st.total

    # Swap values between two indices
    st.update(0, 5)
    st.update(1, 15)
    assert st.total == original_total

    # Redistribute again
    st.update(5, 8)
    st.update(6, 12)
    assert st.total == original_total


def test_alternating_high_low_values():
    """Test alternating between high and low values."""
    st = strust.SumTree(10)

    for i in range(10):
        if i % 2 == 0:
            st.add(1000)
        else:
            st.add(1)

    total = 5 * 1000 + 5 * 1
    assert st.total == total


# ============================================================================
# COPY AND SERIALIZATION CONSISTENCY
# ============================================================================


def test_deepcopy_independence():
    """Test that modifications to copy don't affect original."""
    st1 = strust.SumTree(10)
    for i in range(10):
        st1.add(i)

    st2 = deepcopy(st1)
    original_total = st1.total

    # Modify the copy
    st2.add(100)
    st2.update(0, 50)

    # Original should be unchanged
    assert st1.total == original_total


def test_pickle_multiple_states():
    """Test pickle at different states."""
    import pickle

    st1 = strust.SumTree(10)

    # Pickle empty state
    pickled = pickle.dumps(st1)
    st_empty = pickle.loads(pickled)
    assert st_empty.total == 0.0

    # Add values and pickle again
    for i in range(5):
        st1.add(i)

    pickled = pickle.dumps(st1)
    st_with_values = pickle.loads(pickled)
    assert st_with_values.total == st1.total
    assert st_with_values.capacity == st1.capacity


def test_deepcopy_with_complex_state():
    """Test deepcopy after complex mixed operations."""
    st1 = strust.SumTree(20)

    # Complex operations
    for i in range(10):
        st1.add(random.uniform(10, 100))

    st1.update_batched([0, 1, 2], [50, 50, 50])

    for i in range(5):
        st1.update(i, random.uniform(20, 80))

    st2 = deepcopy(st1)

    assert st1.total == st2.total
    assert st1.capacity == st2.capacity

    for i in range(20):
        assert st1[i] == st2[i]
