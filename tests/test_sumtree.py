import random
from copy import deepcopy
import sumtree_python as stp
import sumtree as strust


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
