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
    for i in range(10_000):
        r = random.randint(0, 100)
        if i % 500 == 0:
            print(".", end="")
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

    