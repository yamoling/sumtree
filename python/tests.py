import random
import sumtree_python as stp
import sumtree as strust


st1 = stp.SumTree(1000)
st2 = strust.SumTree(1000)
for i in range(10_000):
    r = random.randint(0, 100)
    print(f"i={i}\tAdding {r}")
    st1.add(r)
    st2.add(r)
    assert st1.total == st2.total, f"{st1.total}, {st2.total}"

    to_get = random.randint(0, st2.total)
    print(f"Getting {to_get}/{st1.total}")
    g1 = st1.get(to_get)
    g2 = st2.get(to_get)
    assert g1 == g2, f"i={i}\t{g1} != {g2}"
